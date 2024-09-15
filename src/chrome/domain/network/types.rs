use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::chrome::domain::security::types::{CertificateId, MixedContentType, SecurityState};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AlternativeProtocolUsage {
    AlternativeJobWonWithoutRace,
    AlternativeJobWonRace,
    MainJobWonRace,
    MappingMissing,
    Broken,
    DnsAlpnH3JobWonWithoutRace,
    DnsAlpnH3JobWonRace,
    UnspecifiedReason,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BlockedReason {
    Other,
    Csp,
    MixedContent,
    Origin,
    Inspector,
    SubresourceFilter,
    ContentType,
    CoepFrameResourceNeedsCoepHeader,
    CoopSandboxedIframeCannotNavigateToCoopPage,
    CorpNotSameOrigin,
    CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
    CorpNotSameOriginAfterDefaultedToSameOriginByDip,
    CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip,
    CorpNotSameSite,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedResource {
    pub url: String,
    pub r#type: ResourceType,
    pub response: Option<Response>,
    pub body_size: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CertificateTransparencyCompliance {
    Unknown,
    NotCompliant,
    Compliant,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectionType {
    None,
    Cellular2g,
    Cellular3g,
    Cellular4g,
    Bluetooth,
    Ethernet,
    Wifi,
    Wimax,
    Other,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires: i32,
    pub size: i32,
    pub http_only: bool,
    pub secure: bool,
    pub session: bool,
    pub same_site: Option<CookieSameSite>,
    pub priority: CookiePriority,
    pub same_party: bool,
    pub source_scheme: CookieSourceScheme,
    pub source_port: i32,
    pub partition_key: Option<CookiePartitionKey>,
    pub partition_key_opaque: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CookieParam {
    pub name: String,
    pub value: String,
    pub url: Option<String>,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub secure: Option<bool>,
    pub http_only: Option<bool>,
    pub same_site: Option<CookieSameSite>,
    pub expires: Option<i32>,
    pub priority: Option<CookiePriority>,
    pub same_party: Option<bool>,
    pub source_scheme: Option<CookieSourceScheme>,
    pub source_port: Option<i32>,
    pub partition_key: Option<CookiePartitionKey>,
}

#[derive(Serialize, Deserialize)]
pub enum CookieSameSite {
    Strict,
    Lax,
    None,
}

#[derive(Serialize, Deserialize)]
pub enum CorsError {
    DisallowedByMode,
    InvalidResponse,
    WildcardOriginNotAllowed,
    MissingAllowOriginHeader,
    MultipleAllowOriginValues,
    InvalidAllowOriginValue,
    AllowOriginMismatch,
    InvalidAllowCredentials,
    CorsDisabledScheme,
    PreflightInvalidStatus,
    PreflightDisallowedRedirect,
    PreflightWildcardOriginNotAllowed,
    PreflightMissingAllowOriginHeader,
    PreflightMultipleAllowOriginValues,
    PreflightInvalidAllowOriginValue,
    PreflightAllowOriginMismatch,
    PreflightInvalidAllowCredentials,
    PreflightMissingAllowExternal,
    PreflightInvalidAllowExternal,
    PreflightMissingAllowPrivateNetwork,
    PreflightInvalidAllowPrivateNetwork,
    InvalidAllowMethodsPreflightResponse,
    InvalidAllowHeadersPreflightResponse,
    MethodDisallowedByPreflightResponse,
    HeaderDisallowedByPreflightResponse,
    RedirectContainsCredentials,
    InsecurePrivateNetwork,
    InvalidPrivateNetworkAccess,
    UnexpectedPrivateNetworkAccess,
    NoCorsRedirectModeNotFollow,
    PreflightMissingPrivateNetworkAccessId,
    PreflightMissingPrivateNetworkAccessName,
    PrivateNetworkAccessPermissionUnavailable,
    PrivateNetworkAccessPermissionDenied,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorsErrorStatus {
    cors_error: CorsError,
    failed_parameter: String,
}

#[derive(Serialize, Deserialize)]
pub enum ErrorReason {
    Failed, Aborted, TimedOut, AccessDenied, ConnectionClosed, ConnectionReset, ConnectionRefused, ConnectionAborted, ConnectionFailed, NameNotResolved, InternetDisconnected, AddressUnreachable, BlockedByClient, BlockedByResponse
}


#[derive(Serialize, Deserialize)]
pub struct Headers(pub Value);

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InitiatorType {
    Parser,
    Script,
    Preload,
    #[serde(rename = "SignedExchange")]
    SignedExchange,
    Preflight,
    Other,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Initiator {
    pub r#type: InitiatorType,
    pub stack: Option<()>, //todo add stacktrace
    pub url: Option<String>,
    pub line_number: Option<i32>,
    pub column_number: Option<i32>,
    pub request_id: Option<RequestId>,

}

#[derive(Serialize, Deserialize)]
pub struct InterceptionId(pub String);

#[derive(Serialize, Deserialize)]
pub struct LoaderId(pub String);

#[derive(Serialize, Deserialize)]
pub struct MonotonicTime(pub String);

#[derive(Serialize, Deserialize)]
pub struct PostDataEntry {
    bytes: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReferrerPolicy {
    UnsafeUrl,
    NoReferrerWhenDowngrade,
    NoReferrer,
    Origin,
    OriginWhenCrossOrigin,
    SameOrigin,
    StrictOrigin,
    StrictOriginWhenCrossOrigin,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    url: String,
    url_fragment: Option<String>,
    method: String,
    headers: Headers,
    post_data: Option<String>,
    has_post_data: Option<bool>,
    post_data_entries: Option<Vec<PostDataEntry>>,
    mixed_content_type: Option<MixedContentType>,
    initial_priority: ResourcePriority,
    referrer_policy: ReferrerPolicy,
    is_link_preload: Option<bool>,
    trust_token_params: Option<TrustTokenParams>,
    is_same_site: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestId(pub String);

#[derive(Serialize, Deserialize)]
pub enum ResourcePriority {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceTiming {
    pub request_time: i32,
    pub proxy_start: i32,
    pub proxy_end: i32,
    pub dns_start: i32,
    pub dns_end: i32,
    pub connect_start: i32,
    pub connect_end: i32,
    pub ssl_start: i32,
    pub ssl_end: i32,
    pub worker_start: i32,
    pub worker_ready: i32,
    pub worker_fetch_start: i32,
    pub worker_respond_with_settled: i32,
    pub worker_router_evaluation_start: Option<i32>,
    pub worker_cache_lookup_start: Option<i32>,
    pub send_start: i32,
    pub send_end: i32,
    pub push_start: i32,
    pub push_end: i32,
    pub receive_headers_start: i32,
    pub receive_headers_end: i32,
}

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
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub url: String,
    pub status: i32,
    pub status_text: String,
    pub headers: Headers,
    pub headers_text: Option<String>,
    pub mime_type: String,
    pub charset: String,
    pub request_headers: Option<Headers>,
    pub request_headers_text: Option<String>,
    pub connection_refused: bool,
    pub connection_id: i32,
        #[serde(rename = "remoteIPAddress")]
    pub remote_ip_address: Option<String>,
    pub remote_port: Option<i32>,
    pub from_disk_cache: Option<bool>,
    pub from_service_worker: Option<bool>,
    pub from_prefetch_cache: Option<bool>,
    pub from_early_hints: Option<bool>,
    pub service_worker_router_info: Option<ServiceWorkerRouterInfo>,
    pub encoded_data_length: i32,
    pub timing: Option<ResourceTiming>,
    pub service_worker_response_source: Option<ServiceWorkerRouterSource>,
    pub response_time: Option<TimeSinceEpoch>,
    pub cache_storage_cache_name: Option<String>,
    pub protocol: Option<String>,
    pub alternate_protocol_usage: Option<AlternativeProtocolUsage>,
    pub security_state: SecurityState,
    pub security_details: Option<SecurityDetails>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityDetails {
    pub protocol: String,
    pub key_exchange: String,
    pub key_exchange_group: Option<String>,
    pub cipher: String,
    pub mac: Option<String>,
    pub certificate_id: CertificateId,
    pub subject_name: String,
    pub san_list: Vec<String>,
    pub issuer: String,
    pub valid_from: TimeSinceEpoch,
    pub valid_to: TimeSinceEpoch,
    pub signed_certificate_timestamp_list: Vec<SignedCertificateTimestamp>,
    pub certificate_transparency_compliance: CertificateTransparencyCompliance,
    pub server_signature_algorithm: Option<i32>,
    pub encrypted_client_hello: bool,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ServiceWorkerResponseSource {
    CacheStorage,
    HttpCache,
    FallbackCode,
    Network
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ServiceWorkerRouterSource {
    Network,
    Cache,
    FetchEvent,
    RaceNetworkAndFetchHandler,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceWorkerRouterInfo {
    pub rule_id_matched: i32,
    pub matched_source_type: ServiceWorkerRouterSource,
    pub actual_source_type: ServiceWorkerRouterSource,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedCertificateTimestamp {
    pub status: String,
    pub origin: String,
    pub log_description: String,
    pub log_id: String,
    pub timestamp: i32,
    pub hash_algorithm: String,
    pub signature_algorithm: String,
    pub signature_data: String,
}


#[derive(Serialize, Deserialize)]
pub struct TimeSinceEpoch(pub i32);


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketFrame {
    pub opcode: i32,
    pub mask: bool,
    pub payload_data: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketRequest {
    pub headers: Headers,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketResponse {
    pub status: i32,
    pub status_text: String,
    pub headers: Headers,
    pub headers_text: Option<String>,
    pub request_headers: Option<Headers>,
    pub request_headers_text: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub enum CookiePriority {
    Low, Medium, High
}

#[derive(Serialize, Deserialize)]
pub enum CookieSourceScheme {
    Unset, NonSecure, Secure
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CookiePartitionKey {
    top_level_site: String,
    has_cross_site_ancestor: bool,
}


#[derive(Serialize, Deserialize)]
pub enum TrustTokenOperationType {
    Issuance, Redemption, Signing
}

#[derive(Serialize, Deserialize)]
pub enum RefreshPolicy {
    UseCached,
    Refresh,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrustTokenParams {
    operation: TrustTokenOperationType,
    refresh_policy: RefreshPolicy,
    issuers: Option<Vec<String>>
}
