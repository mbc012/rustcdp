use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::chrome::domain::{browser, page};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionID(pub String);

impl SessionID {
    // todo: Move as_str/as_string to a trait?

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_string(&self) -> &String {
        &self.0
    }
}

impl From<SessionID> for Value {
    fn from(s: SessionID) -> Value {
        Value::from(s.0.clone())
    }
}

impl From<&SessionID> for Value {
    fn from(s: &SessionID) -> Value {
        Value::from(s.0.clone())
    }
}


#[derive(Serialize, Deserialize)]
pub struct TargetID(pub String);

impl From<TargetID> for Value {
    fn from(target: TargetID) -> Value {
        Value::from(target.0.clone())
    }
}

impl From<&TargetID> for Value {
    fn from(target: &TargetID) -> Value {
        Value::from(target.0.clone())
    }
}



#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct TargetInfo {
    pub target_id:              TargetID,
    pub r#type:                 TargetInfoTypes,
    pub title:                  String,
    pub url:                    String,
    pub attached:               bool,
    pub opener_id:              Option<TargetID>,
    pub can_access_opener:      Option<bool>,
    pub opener_frame_id:        Option<page::types::FrameId>,
    pub browser_context_id:     Option<browser::types::BrowserContextID>,
    pub subtype:                Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TargetInfoTypes {
    Tab,
    Page,
    Iframe,
    Worker,
    SharedWorker,
    ServiceWorker,
    Worklet,
    SharedStorageWorklet,
    Browser,
    Webview,
    Other,
    AuctionWorklet,
    AssistiveTechnology,
}

#[derive(Serialize, Deserialize)]
pub struct FilterEntry {
    pub exclude: Option<bool>,
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RemoteLocation {
    pub host: String,
    pub port: u32,
}

#[derive(Serialize, Deserialize)]
pub struct TargetFilter(pub Vec<FilterEntry>);

