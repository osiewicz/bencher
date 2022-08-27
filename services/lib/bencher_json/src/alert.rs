use ordered_float::OrderedFloat;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonAlert {
    pub uuid:      Uuid,
    pub report:    Uuid,
    pub perf:      Option<Uuid>,
    pub threshold: Uuid,
    pub statistic: Uuid,
    pub side:      JsonSide,
    pub boundary:  OrderedFloat<f64>,
    pub outlier:   OrderedFloat<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum JsonSide {
    Left,
    Right,
}