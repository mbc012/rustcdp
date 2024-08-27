use serde::{Deserialize, Serialize};
use crate::chrome::domain::{network, runtime};

#[derive(Serialize, Deserialize)]
pub struct AppManifestError {
    message: String,
    critical: i32,
    line: i32,
    column: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DialogType {
    Alert,
    Confirm,
    Prompt,
    BeforeUnload
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    id: FrameId,
    parent_id: Option<FrameId>,
    loader_id: network::types::LoaderId,
    name: Option<String>,
    url: String,
    url_fragment: Option<String>,
    domain_and_registry: String,
    security_origin: String,
    mime_type: String,
    unreachable_url: Option<String>,
    ad_frame_status: Option<AdFrameStatus>,
    secure_context_type: SecureContextType,
    cross_origin_isolated_context_type: CrossOriginIsolatedContextType,
    #[serde(rename = "gatedAPIFeatures")]
    gated_api_features: Vec<GatedAPIFeatures>
}


#[derive(Serialize, Deserialize)]
pub struct FrameId(pub String);


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameTree {
    frame: Frame,
    child_frames: Option<Vec<FrameTree>>
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutViewport {
    page_x: i32,
    page_y: i32,
    client_width: i32,
    client_height: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEntry {
    id: i32,
    url: String,
    #[serde(rename = "userTypedURL")]
    user_typed_url: String,
    title: String,
    transition_type: TransitionType
}

#[derive(Serialize, Deserialize)]
pub struct ScriptIdentifier(pub String);

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionType {
    Link,
    Typed,
    AddressBar,
    AutoBookmark,
    AutoSubframe,
    ManualSubframe,
    Generated,
    AutoToplevel,
    FormSubmit,
    Reload,
    Keyword,
    KeywordGenerated,
    Other
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Viewport {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    scale: f32, // TODO: EVAL TYPES ARE CORRECT.
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualViewport {
    offset_x: i32,
    offset_y: i32,
    page_x: i32,
    page_y: i32,
    client_width: i32,
    client_height: i32,
    scale: f32,
    zoom: Option<f32>, // TODO: EVAL TYPES ARE CORRECT
}

#[derive(Serialize, Deserialize)]
pub enum AdFrameExplanation {
    ParentIsAd,
    CreatedByAdScript,
    MatchedBlockingRule,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdFrameStatus {
    ad_frame_type: AdFrameType,
    explanations: Option<Vec<AdFrameExplanation>>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AdFrameType {
    None,
    Child,
    Root,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdScriptId {
    script_id: runtime::types::ScriptId,
    debugger_id: runtime::types::UniqueDebuggerId,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct AppManifestParsedProperties {
    scope: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AutoResponseMode {
    None,
    AutoAccept,
    AutoReject,
    AutoOutput,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackForwardCacheBlockingDetails {
    url: Option<String>,
    function: Option<String>,
    line_number: i32,
    column_number: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackForwardCacheNotRestoredExplanation {
    r#type: BackForwardCacheNotRestoredReasonType,
    reason: BackForwardCacheNotRestoredReason,
    context: Option<String>,
    details: Option<Vec<BackForwardCacheBlockingDetails>>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackForwardCacheNotRestoredExplanationTree {
    url: String,
    explanations: Vec<BackForwardCacheNotRestoredExplanation>,
    children: Vec<BackForwardCacheNotRestoredExplanationTree>
}

#[derive(Serialize, Deserialize)]
pub enum BackForwardCacheNotRestoredReason {
    NotPrimaryMainFrame,
    BackForwardCacheDisabled,
    RelatedActiveContentsExist,
    HTTPStatusNotOK,
    SchemeNotHTTPOrHTTPS,
    Loading,
    WasGrantedMediaAccess,
    DisableForRenderFrameHostCalled,
    DomainNotAllowed,
    HTTPMethodNotGET,
    SubframeIsNavigating,
    Timeout,
    CacheLimit,
    JavaScriptExecution,
    RendererProcessKilled,
    RendererProcessCrashed,
    SchedulerTrackedFeatureUsed,
    ConflictingBrowsingInstance,
    CacheFlushed,
    ServiceWorkerVersionActivation,
    SessionRestored,
    ServiceWorkerPostMessage,
    EnteredBackForwardCacheBeforeServiceWorkerHostAdded,
    RenderFrameHostReused_SameSite,
    RenderFrameHostReused_CrossSite,
    ServiceWorkerClaim,
    IgnoreEventAndEvict,
    HaveInnerContents,
    TimeoutPuttingInCache,
    BackForwardCacheDisabledByLowMemory,
    BackForwardCacheDisabledByCommandLine,
    NetworkRequestDatapipeDrainedAsBytesConsumer,
    NetworkRequestRedirected,
    NetworkRequestTimeout,
    NetworkExceedsBufferLimit,
    NavigationCancelledWhileRestoring,
    NotMostRecentNavigationEntry,
    BackForwardCacheDisabledForPrerender,
    UserAgentOverrideDiffers,
    ForegroundCacheLimit,
    BrowsingInstanceNotSwapped,
    BackForwardCacheDisabledForDelegate,
    UnloadHandlerExistsInMainFrame,
    UnloadHandlerExistsInSubFrame,
    ServiceWorkerUnregistration,
    CacheControlNoStore,
    CacheControlNoStoreCookieModified,
    CacheControlNoStoreHTTPOnlyCookieModified,
    NoResponseHead,
    Unknown,
    ActivationNavigationsDisallowedForBug1234857,
    ErrorDocument,
    FencedFramesEmbedder,
    CookieDisabled,
    HTTPAuthRequired,
    CookieFlushed,
    BroadcastChannelOnMessage,
    WebViewSettingsChanged,
    WebViewJavaScriptObjectChanged,
    WebViewMessageListenerInjected,
    WebViewSafeBrowsingAllowlistChanged,
    WebViewDocumentStartJavascriptChanged,
    WebSocket,
    WebTransport,
    WebRTC,
    MainResourceHasCacheControlNoStore,
    MainResourceHasCacheControlNoCache,
    SubresourceHasCacheControlNoStore,
    SubresourceHasCacheControlNoCache,
    ContainsPlugins,
    DocumentLoaded,
    OutstandingNetworkRequestOthers,
    RequestedMIDIPermission,
    RequestedAudioCapturePermission,
    RequestedVideoCapturePermission,
    RequestedBackForwardCacheBlockedSensors,
    RequestedBackgroundWorkPermission,
    BroadcastChannel,
    WebXR,
    SharedWorker,
    WebLocks,
    WebHID,
    WebShare,
    RequestedStorageAccessGrant,
    WebNfc,
    OutstandingNetworkRequestFetch,
    OutstandingNetworkRequestXHR,
    AppBanner,
    Printing,
    WebDatabase,
    PictureInPicture,
    SpeechRecognizer,
    IdleManager,
    PaymentManager,
    SpeechSynthesis,
    KeyboardLock,
    WebOTPService,
    OutstandingNetworkRequestDirectSocket,
    InjectedJavascript,
    InjectedStyleSheet,
    KeepaliveRequest,
    IndexedDBEvent,
    Dummy,
    JsNetworkRequestReceivedCacheControlNoStoreResource,
    WebRTCSticky,
    WebTransportSticky,
    WebSocketSticky,
    SmartCard,
    LiveMediaStreamTrack,
    UnloadHandler,
    ParserAborted,
    ContentSecurityHandler,
    ContentWebAuthenticationAPI,
    ContentFileChooser,
    ContentSerial,
    ContentFileSystemAccess,
    ContentMediaDevicesDispatcherHost,
    ContentWebBluetooth,
    ContentWebUSB,
    ContentMediaSessionService,
    ContentScreenReader,
    EmbedderPopupBlockerTabHelper,
    EmbedderSafeBrowsingTriggeredPopupBlocker,
    EmbedderSafeBrowsingThreatDetails,
    EmbedderAppBannerManager,
    EmbedderDomDistillerViewerSource,
    EmbedderDomDistillerSelfDeletingRequestDelegate,
    EmbedderOomInterventionTabHelper,
    EmbedderOfflinePage,
    EmbedderChromePasswordManagerClientBindCredentialManager,
    EmbedderPermissionRequestManager,
    EmbedderModalDialog,
    EmbedderExtensions,
    EmbedderExtensionMessaging,
    EmbedderExtensionMessagingForOpenPort,
    EmbedderExtensionSentMessageToCachedFrame,
    RequestedByWebViewClient
}

#[derive(Serialize, Deserialize)]
pub enum BackForwardCacheNotRestoredReasonType {
    SupportPending,
    PageSupportNeeded,
    Circumstantial,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientNavigationDisposition {
    CurrentTab,
    NewTab,
    NewWindow,
    Download,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientNavigationReason {
    AnchorClick,
    FormSubmissionGet,
    FormSubmissionPost,
    HttpHeaderRefresh,
    InitialFrameNavigation,
    MetaTagRefresh,
    Other,
    PageBlockInterstitial,
    Reload,
    ScriptInitiated,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompilationCacheParams {
    url: String,
    eager: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum CrossOriginIsolatedContextType {
    Isolated,
    NotIsolated,
    NotIsolatedFeatureDisabled,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileFilter {
    name: Option<String>,
    accepts: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileHandler {
    action: String,
    name: String,
    icons: Option<Vec<ImageResource>>,
    accepts: Option<Vec<FileFilter>>,
    launch_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontFamilies {
    standard: Option<String>,
    fixed: Option<String>,
    serif: Option<String>,
    sans_serif: Option<String>,
    cursive: Option<String>,
    fantasy: Option<String>,
    math: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontSizes {
    standard: Option<i32>,
    fixed: Option<i32>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameResource {
    url: String,
    r#type: network::types::ResourceType,
    mime_type: String,
    last_modified: Option<network::types::TimeSinceEpoch>,
    content_size: Option<i32>,
    failed: Option<bool>,
    canceled: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameResourceTree {
    frame: Frame,
    child_frames: Option<Vec<FrameResourceTree>>,
    resources: Vec<FrameResource>,
}

#[derive(Serialize, Deserialize)]
pub enum GatedAPIFeatures {
    SharedArrayBuffers,
    SharedArrayBuffersTransferAllowed,
    PerformanceMeasureMemory,
    PerformanceProfile,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageResource {
    url: String,
    sizes: Option<String>,
    r#type: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallabilityError {
    error_id: String,
    error_arguments: Vec<InstallabilityErrorArgument>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallabilityErrorArgument {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchHandler {
    client_mode: String,
}


#[derive(Serialize, Deserialize)]
pub enum NavigationType {
    Navigation,
    BackForwardCacheRestore
}

// TODO ORIGIN TRIAL STUFF
// https://chromedevtools.github.io/devtools-protocol/tot/Page/#type-OriginTrial


#[derive(Serialize, Deserialize)]
pub enum SecureContextType {
    Secure,
    SecureLocalhost,
    InsecureScheme,
    InsecureAncestor,
}