use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::time::Duration;
use crate::chrome::browser::message::SocketMessage;
use crate::chrome::state::ChromeState;

impl ChromeState {

    pub(crate) fn browser_message_handler(
        state: Arc<Mutex<ChromeState>>,
        browser_message_receiver: Receiver<SocketMessage>,
        stop_flag: Arc<AtomicBool>
    ) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            loop {
                if stop_flag.load(Ordering::Relaxed) {
                    break
                }

                let incoming_message = browser_message_receiver.recv().expect("[State Listener] Failed to receive SocketMessage.");
                println!("[State Listener] {}", incoming_message.stringify()); // TODO

                std::thread::sleep(Duration::from_millis(10))
            }
        })
    }

}

