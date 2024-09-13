use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserVersion {
    pub protocol_version: String,
    pub product: String,
    pub revision: String,
    pub user_agent: String,
    pub js_version: String,
}


#[derive(Serialize, Deserialize)]
pub struct Bounds {
    pub left: Option<i32>, // TODO: CHECK TYPE i32
    pub top: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub window_state: Option<WindowState>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BrowserCommandId {
    OpenTabSearch,
    CloseTabSearch,
}

#[derive(Serialize, Deserialize)]
pub struct BrowserContextID(pub String);

#[derive(Serialize, Deserialize)]
pub struct Bucket {
    pub low: i32,
    pub high: i32,
    pub count: i32, // TODO: CHECK TYPE i32
}

#[derive(Serialize, Deserialize)]
pub struct Histogram {
    pub name: String,
    pub sum: i32,
    pub count: i32,
    pub buckets: Vec<Bucket>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionDescriptor {
    pub name: String,
    pub sysex: Option<bool>,
    pub user_visible_only: Option<bool>,
    pub allow_without_sanitization: Option<bool>,
    pub allow_without_gesture: Option<bool>,
    pub pan_tilt_zoom: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PermissionSetting {
    Granted,
    Denied,
    Prompt
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionType {
    AccessibilityEvents,
    AudioCapture,
    BackgroundSync,
    BackgroundFetch,
    CapturedSurfaceControl,
    ClipboardReadWrite,
    ClipboardSanitizedWrite,
    DisplayCapture,
    DurableStorage,
    Flash,
    Geolocation,
    IdleDetection,
    LocalFonts,
    Midi,
    MidiSysex,
    Nfc,
    Notifications,
    PaymentHandler,
    PeriodicBackgroundSync,
    ProtectedMediaIdentifier,
    Sensors,
    StorageAccess,
    SpeakerSelection,
    TopLevelStorageAccess,
    VideoCapture,
    VideoCapturePanTiltZoom,
    WakeLockScreen,
    WakeLockSystem,
    WindowManagement,
}


#[derive(Serialize, Deserialize)]
pub struct WindowID(pub String);

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}