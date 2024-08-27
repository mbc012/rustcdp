use std::collections::HashMap;
use std::marker::PhantomData;
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use crate::chrome::Chrome;
use crate::chrome::domain::target::types::SessionID;


impl Chrome {
    pub(crate) fn send_message(&mut self, UserCallMessage { method, params, session_id }: UserCallMessage) -> u32 {
        let mut guard = self.user_call_registry.lock().expect("Failed to lock ucr.");
        let index = guard.increment_index();
        let socket_msg = SocketMessage::new(index, method, params).session_id(session_id);

        println!("[OUTGOING]: {}", socket_msg.stringify());

        guard.insert(socket_msg.clone());
        self.browser.send_message(socket_msg);
        index
    }
}



#[derive(Serialize, Deserialize)]
pub struct UserCallMessage {
    method: String,
    params: HashMap<String, Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_id: Option<SessionID>
}


impl UserCallMessage {
    pub fn new<S: Into<String>>(method: S) -> Self {
        Self {
            method: method.into(),
            params: HashMap::new(),
            session_id: None
        }
    }

    pub fn set_params(mut self, p: HashMap<String, Value>)  -> Self {
        self.params = p;
        self
    }

    pub fn set_session_id(mut self, s: SessionID) -> Self {
        self.session_id = Some(s);
        self
    }

}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SocketMessage {
    id:     Option<u32>,
    method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_id: Option<SessionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<HashMap<String, Value>>,
    #[serde(skip_serializing)]
    result: Option<HashMap<String, Value>>,
    #[serde(skip_serializing)]
    error:  Option<HashMap<String, Value>>,
}


impl SocketMessage {
    pub fn new(id: u32, method: String, params: HashMap<String, Value>) -> Self {
        if params.is_empty() {
            Self { id: Some(id), method: Some(method), session_id: None, params: None, result: None, error: None }
        } else {
            Self { id: Some(id), method: Some(method), session_id: None, params: Some(params), result: None, error: None }
        }
    }

    pub fn session_id(mut self, id: Option<SessionID>) -> Self {
        self.session_id = id;
        self
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap()
    }

    pub fn has_method(&self) -> bool {
        self.method.is_some()
    }

    pub fn get_data(&self) -> HashMap<String, Value> {
        // Checks the result, error and params and returns if not returns empty;
        if let Some(result) = &self.result {
            if !result.is_empty() {
                return result.clone();
            }
        }

        if let Some(error) = &self.error {
            if !error.is_empty() {
                return error.clone();
            }
        }

        if let Some(params) = &self.params {
            if !params.is_empty() {
                return params.clone();
            }
        }

        HashMap::new()
    }

    pub fn get_method(&self) -> Option<String> {
        match &self.method {
            Some(val) => Some(val.to_string()),
            _ => None
        }
    }

    pub fn stringify(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn params_convert<T>(&self) -> T where T: for<'de> Deserialize<'de> {
        let val = self.params
            .clone()
            .expect("Unable to convert params: No params")
            .values()
            .next()
            .expect("Unable to convert params: No param value")
            .clone();
        serde_json::from_value(val)
            .expect("Unable to convert value to type.")

    }

}

impl From<String> for SocketMessage {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap()
    }
}
