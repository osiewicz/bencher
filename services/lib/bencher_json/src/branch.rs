#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

use crate::ResourceId;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonNewBranch {
    pub project: ResourceId,
    pub name:    String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug:    Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonBranch {
    pub uuid:         Uuid,
    pub project_uuid: Uuid,
    pub name:         String,
    pub slug:         String,
}