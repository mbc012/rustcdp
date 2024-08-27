use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ScriptId(pub String);

#[derive(Serialize, Deserialize)]
pub struct UniqueDebuggerId(pub String);


