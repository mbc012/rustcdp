use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use serde_json::Value;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};


use crate::chrome::browser::ChromeBrowser;
use crate::chrome::browser::message::SocketMessage;


impl ChromeBrowser {

    pub(crate) fn send_message(&mut self, socket_message: SocketMessage) {
        let msg = socket_message.stringify();
        let mut guard = self.websocket_connection.lock().expect("Failed to lock websocket connection.");
        guard.send(msg.into()).expect("Failed to send message to websocket connection.");
        self.websocket_handler_thread.thread().unpark();
    }

    pub(crate) fn connect_websocket(ws_uri: &url::Url) -> Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>> {
        let mut client = tungstenite::client::connect(ws_uri.as_str()).expect("Failed to connect to websocket.");

        match client.0.get_mut() {
            MaybeTlsStream::Plain(stream) => stream.set_read_timeout(Some(Duration::from_millis(500))),
            _ => panic!("Unsupported stream type."),
        }.expect("Failed to set timeout for websocket stream.");

        let ws = Arc::new(Mutex::new(client.0));
        ws
    }



    /// Handles incoming messages from websocket connection
    pub(crate) fn websocket_message_handler(
        socket_connection: Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>,
        browser_message_sender: std::sync::mpsc::Sender<SocketMessage>,
        user_message_sender: std::sync::mpsc::Sender<SocketMessage>,
    ) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            loop {
                let mut guard = socket_connection.lock().unwrap();

                let msg = match guard.read() {
                    Ok(m) => {
                        m
                    },

                    Err(err) => {
                        drop(guard);
                        std::thread::sleep(Duration::from_millis(100));
                        continue;
                    },
                };


                match msg {
                    Message::Text(txt) => {
                        let parse: SocketMessage = serde_json::from_str(&txt).expect("Failed to parse message into SocketMessage");
                        //
                        let p = &parse.stringify();
                        println!("{}", p);

                        if parse.has_id() {
                            // User issued command
                            let payload = parse.clone();
                            user_message_sender.send(payload).expect("Failed to send received user message internally.");
                        }

                        if parse.has_method() {
                            // Handle events
                            let payload = parse.clone();
                            browser_message_sender.send(payload).expect("Failed to send received socket message internally.");
                        }
                    },
                    _ => {
                        println!("Non-text message received!");
                    }
                }

                drop(guard);
                std::thread::sleep(Duration::from_millis(10));
            }
        })
    }
}