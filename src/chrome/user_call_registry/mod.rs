use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;
use serde_json::Value;
use crate::chrome::browser::message::SocketMessage;
use crate::chrome::user_call_registry::entry::UserCallEntry;

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
            timeout: 5, // seconds
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

    pub fn retrieve(&self, idx: u32) -> Option<Vec<HashMap<String, Value>>> {  // TODO Change ret type
        if self.entries.contains_key(&idx) {
            let find = self.entries.get(&idx);
            if find.is_some() && find.unwrap().has_response() {
                Some(find.unwrap().get_data())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn wait(&self, idx: u32) -> Option<HashMap<String, Value>> {
        let mut call = None;
        let mut current = 0;

        while call.is_none() {
            call = self.retrieve(idx);
            if self.timeout != 0 {
                if current >= self.timeout {
                    break;
                }
                current += 1;
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        Some(call.unwrap().first().unwrap().clone())
    }

    pub fn increment_index(&mut self) -> u32 {
        let c = self.index;
        self.index = self.index + 1;
        c
    }
}