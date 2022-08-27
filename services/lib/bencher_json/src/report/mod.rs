#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize,
};

pub mod data;
pub mod new;

pub use data::JsonReport;
pub use new::{
    latency::JsonLatency,
    metrics_map::JsonMetricsMap,
    min_max_avg::JsonMinMaxAvg,
    throughput::JsonThroughput,
    JsonNewReport,
};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum JsonAdapter {
    Json,
    Rust,
}