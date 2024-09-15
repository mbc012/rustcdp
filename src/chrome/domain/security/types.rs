use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CertificateErrorAction {
    Continue,
    Cancel,
}

#[derive(Serialize, Deserialize)]
pub struct CertificateId(pub i32);

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MixedContentType {
    Blockable,
    OptionallyBlockable,
    None,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SecurityState {
    Unknown,
    Neutral,
    Insecure,
    Secure,
    Info,
    InsecureBroken,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityStateExplanation {
    pub security_state: SecurityState,
    pub title: String,
    pub summary: String,
    pub description: String,
    pub mixed_content_type: MixedContentType,
    pub certificate: Vec<String>,
    pub recommendations: Option<Vec<String>>,
}

