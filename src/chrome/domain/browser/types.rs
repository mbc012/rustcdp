use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Bounds {
    left: Option<i32>, // TODO: CHECK TYPE i32
    top: Option<i32>,
    width: Option<i32>,
    height: Option<i32>,
    window_state: Option<WindowState>
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
    low: i32,
    high: i32,
    count: i32, // TODO: CHECK TYPE i32
}

#[derive(Serialize, Deserialize)]
pub struct Histogram {
    name: String,
    sum: i32,
    count: i32,
    buckets: Vec<Bucket>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionDescriptor {
    name: String,
    sysex: Option<bool>,
    user_visible_only: Option<bool>,
    allow_without_sanitization: Option<bool>,
    allow_without_gesture: Option<bool>,
    pan_tilt_zoom: Option<bool>,
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