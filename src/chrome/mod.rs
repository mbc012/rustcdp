use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::chrome::browser::message::{SocketMessage};

pub(crate) mod browser;
mod state;
mod user_call_registry;
pub mod domain;


pub use browser::message::UserCallMessage;



pub struct Chrome {
    state: Arc<Mutex<state::ChromeState>>,
    browser: browser::ChromeBrowser,
    user_call_registry: Arc<Mutex<user_call_registry::UserCallRegistry>>,
    state_thread: std::thread::JoinHandle<()>,
    ucr_thread: std::thread::JoinHandle<()>,
    stop_flag: Arc<AtomicBool>,
}

impl Chrome {
    pub fn new() -> Chrome {
        // stop flag (thread control)
        let stop_flag = Arc::new(AtomicBool::new(false));

        // Create mpsc channels
        let (browser_message_sender, browser_message_receiver) = std::sync::mpsc::channel::<SocketMessage>();
        let (user_message_sender, user_message_receiver) = std::sync::mpsc::channel::<SocketMessage>();

        // Initiate chrome instance
        let browser = browser::ChromeBrowser::new(browser_message_sender, user_message_sender);
        let (ucr, ucr_thread) = user_call_registry::UserCallRegistry::new(user_message_receiver, Arc::clone(&stop_flag));
        let (state, state_thread) = state::ChromeState::new(browser_message_receiver, Arc::clone(&stop_flag));

        Self {
            state: state,
            browser: browser,
            user_call_registry: ucr,
            state_thread: state_thread,
            ucr_thread: ucr_thread,
            stop_flag: stop_flag,
        }
    }
}

impl Drop for Chrome {
    fn drop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }
}




