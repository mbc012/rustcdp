use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::time::Duration;


use crate::chrome::browser::message::SocketMessage;
use crate::chrome::user_call_registry::UserCallRegistry;

impl UserCallRegistry {

    pub(crate) fn user_call_message_handler(
        ucr: Arc<Mutex<UserCallRegistry>>,
        user_call_receiver: Receiver<SocketMessage>,
        stop_flag: Arc<AtomicBool>,
    ) -> std ::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            loop {
                if stop_flag.load(Ordering::Relaxed) {
                    break
                }

                let incoming_message = user_call_receiver.recv().expect("[UserCall Listener] Failed to receive SocketMessage");
                println!("[UserCall Listener] {}", incoming_message.stringify()); // TODO



                std::thread::sleep(Duration::from_millis(10))
            }
        })
    }



}
