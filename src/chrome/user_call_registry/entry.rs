use std::collections::HashMap;
use serde_json::Value;
use crate::chrome::browser::message::SocketMessage;

#[derive(Debug)]
pub struct UserCallEntry {
    id: u32,
    updated: bool,
    data: Value
}

impl UserCallEntry {
    pub fn existing_call(&mut self) {
        // There is an existing call entry in the UCR, therefore reflect in UCE
        // (this is a response to a usercall)
        self.updated = true;
    }

    pub fn has_response(&self) -> bool {
        self.updated
    }

    pub fn get_data(&self) -> Value {
        self.data.clone()
    }
}

impl From<SocketMessage> for UserCallEntry {
    fn from(value: SocketMessage) -> Self {
        Self {
            id: value.get_id(),
            updated: false,
            data: value.get_result().unwrap_or(Value::Null),
        }
    }
}


