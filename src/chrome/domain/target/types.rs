use serde::{Deserialize, Serialize};
use crate::chrome::domain::{browser, page};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionID(pub String);

#[derive(Serialize, Deserialize)]
pub struct TargetID(pub String);

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct TargetInfo {
    target_id:              TargetID,
    r#type:                 TargetInfoTypes,
    title:                  String,
    url:                    String,
    attached:               bool,
    opener_id:              Option<TargetID>,
    can_access_opener:      Option<bool>,
    opener_frame_id:        Option<page::types::FrameId>,
    browser_context_id:     Option<browser::types::BrowserContextID>,
    subtype:                Option<String>,
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
    exclude: Option<bool>,
    r#type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RemoteLocation {
    host: String,
    port: u32,
}

#[derive(Serialize, Deserialize)]
pub struct TargetFilter(pub Vec<FilterEntry>);

