use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoaderId(pub String);


#[derive(Serialize, Deserialize)]
pub enum ResourceType {
    Document,
    Stylesheet,
    Image,
    Media,
    Font,
    Script,
    TextTrack,
    XHR,
    Fetch,
    Prefetch,
    EventSource,
    WebSocket,
    Manifest,
    SignedExchange,
    Ping,
    CSPViolationReport,
    Preflight,
    Other,
}

#[derive(Serialize, Deserialize)]
pub struct TimeSinceEpoch(pub i32);


