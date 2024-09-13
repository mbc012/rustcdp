use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;
use serde_json::Value;
use crate::chrome::browser::message::SocketMessage;
use crate::chrome::Chrome;
use crate::chrome::user_call_registry::entry::UserCallEntry;
use crate::error::{Result, Error};

mod entry;
mod listener;

pub struct UserCallRegistry {
    index: u32,
    timeout: u8,
    entries: HashMap<u32, UserCallEntry>,
}

impl UserCallRegistry {
    pub fn new(user_message_receiver: Receiver<SocketMessage>, stop_flag: Arc<AtomicBool>) -> (Arc<Mutex<UserCallRegistry>>, std::thread::JoinHandle<()>) {
        let registry = Self {
            entries: HashMap::new(),
            index: 1,
            timeout: 10, // seconds
        };

        let arc_registry = Arc::new(Mutex::new(registry));

        let listener_thread = UserCallRegistry::user_call_message_handler(
            Arc::clone(&arc_registry),
            user_message_receiver,
            stop_flag
        );

        (arc_registry, listener_thread)
    }

    pub fn set_timeout(&mut self, timeout: u8) {
        self.timeout = timeout;
    }

    pub fn insert(&mut self, socket_message: SocketMessage) -> u32 {
        // Creates a new call or overwrites an existing one
        let sm_id = socket_message.get_id();
        let mut uce: UserCallEntry = socket_message.into();

        if self.entries.contains_key(&sm_id) {
            uce.existing_call();
        }

        self.entries.insert(sm_id, uce);
        sm_id
    }

    pub fn retrieve(&self, id: u32) -> Option<Value> {  // TODO Change ret type
        if self.entries.contains_key(&id) {
            let find = self.entries.get(&id);
            if find.is_some() && find?.has_response() {
                return Some(find?.get_data())
            }
        }
        None
    }

    pub fn increment_index(&mut self) -> u32 {
        let c = self.index;
        self.index = self.index + 1;
        c
    }
}


impl Chrome {
    pub fn wait_ucr(&self, id: u32) -> Result<Value> {
        let mut call = None;
        let mut t_set = false;
        let mut timeout: u32 = 0; //ms
        let mut current: u32 = 0; //ms

        while call.is_none() {
            let mut guard = self.user_call_registry.lock().unwrap();

            if !t_set {
                timeout = guard.timeout as u32 * 1000; // s to ms
                t_set = true;
            }

            call = guard.retrieve(id);
            drop(guard);

            if timeout != 0 {
                if current >= timeout {
                    return Error::TimeoutExceeded {
                        current_timer_ms: current as u64,
                        timeout_ms: timeout as u64,
                    }.as_err();
                }

                current += 100;
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        Ok(call.unwrap())
    }
}

