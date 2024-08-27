use std::path::PathBuf;
use std::borrow::BorrowMut;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;


use crate::chrome::browser::message::SocketMessage;

mod spawn;
mod websocket;
pub(crate) mod message;




pub struct ChromeBrowser {
    process: std::process::Child,
    debug_websocket: url::Url,
    websocket_connection: Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>,
    websocket_handler_thread: std::thread::JoinHandle<()>,

}


impl ChromeBrowser {
    pub fn new(browser_message_sender: Sender<SocketMessage>, user_message_sender: Sender<SocketMessage>) -> Self {
        // Default chrome location
        let chrome_location = PathBuf::from("C:/Program Files/Chromium/Application/chrome.exe");
        // Default arguments
        let arguments = vec![
            "--verbose".into(),
            "--log-level=0".into(),
            "--no-first-run".into(),
            "--disable-audio-output".into()
        ];
        // Spawn process and extract socket url
        let mut process = ChromeBrowser::spawn_process(chrome_location, arguments);
        let socket_url = ChromeBrowser::read_process(process.borrow_mut()).expect("Failed to find ws url.");
        let debug_websocket = url::Url::parse(socket_url.as_str()).expect("Failed to parse ws url.");


        // Websocket connection and handler thread
        let websocket_connection = ChromeBrowser::connect_websocket(&debug_websocket);
        let websocket_handler_thread = ChromeBrowser::websocket_message_handler(
            Arc::clone(&websocket_connection),
            browser_message_sender,
            user_message_sender
        );

        // Browser receiving...

        Self {
            process,
            debug_websocket,
            websocket_connection,
            websocket_handler_thread
        }
    }
}
