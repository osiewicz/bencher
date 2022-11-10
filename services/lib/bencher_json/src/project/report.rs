use chrono::{DateTime, Utc};
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::JsonAdapter;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonNewReport {
    pub branch: Uuid,
    pub hash: Option<String>,
    pub testbed: Uuid,
    pub adapter: JsonAdapter,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub results: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonReport {
    pub uuid: Uuid,
    pub user: Uuid,
    pub version: Uuid,
    pub testbed: Uuid,
    pub adapter: JsonAdapter,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub results: JsonReportResults,
    pub alerts: JsonReportAlerts,
}

pub type JsonReportResults = Vec<JsonReportResult>;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonReportResult(pub Uuid);

impl From<Uuid> for JsonReportResult {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

pub type JsonReportAlerts = Vec<JsonReportAlert>;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonReportAlert(pub Uuid);

impl From<Uuid> for JsonReportAlert {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}
