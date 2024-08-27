use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;
use crate::chrome::browser::ChromeBrowser;
use crate::chrome::browser::message::SocketMessage;

mod window;
mod target;
mod listener;


pub struct ChromeState {
    targets: Vec<String> // TODO: CHANGE TYPE
    // TODO: Add chrome state here
}


impl ChromeState {
    pub fn new(browser_message_receiver: Receiver<SocketMessage>, stop_flag: Arc<AtomicBool>) -> (Arc<Mutex<ChromeState>>, std::thread::JoinHandle<()>) {
        let state = Arc::new(Mutex::new(Self {
            targets: Vec::new(),
        }));

        let thread = ChromeState::browser_message_handler(
            Arc::clone(&state),
            browser_message_receiver,
            stop_flag
        );

        (state, thread)
    }
}


