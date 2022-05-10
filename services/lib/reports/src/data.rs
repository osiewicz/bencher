#[cfg(feature = "wasm")]
use std::collections::HashSet;

use chrono::{DateTime, Utc};
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::Serialize;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use crate::Reports;

#[derive(Debug, Default, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Data {
    x: Vec<DateTime<Utc>>,
    y: Vec<u64>,
    z: Vec<String>,
}

#[cfg(feature = "wasm")]
struct Datum {
    x: DateTime<Utc>,
    y: u64,
    z: String,
}

#[cfg(feature = "wasm")]
impl Data {
    fn push(&mut self, datum: Datum) {
        self.x.push(datum.x);
        self.y.push(datum.y);
        self.z.push(datum.z);
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct InventoryData {
    inventory: JsValue,
    data: JsValue,
}

#[cfg(feature = "wasm")]
type Inventory = Vec<String>;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl InventoryData {
    pub(crate) fn new_latency(reports: &Reports) -> Self {
        let (inventory, data) = Self::latency(reports);
        Self {
            inventory: JsValue::from_serde(&inventory)
                .expect(&format!("Failed to serialize latency inventory JSON")),
            data: JsValue::from_serde(&data)
                .expect(&format!("Failed to serialize latency data JSON")),
        }
    }

    fn latency(reports: &Reports) -> (Inventory, Data) {
        let mut names_set = HashSet::new();
        let mut data = Data::default();
        for (_, report) in reports.as_ref().iter() {
            for (name, metric) in report.metrics.iter() {
                if let Some(latency) = &metric.latency() {
                    names_set.insert(name.clone());
                    data.push(Datum {
                        x: report.date_time.clone(),
                        y: latency.duration.as_micros() as u64,
                        z: name.clone(),
                    })
                }
            }
        }
        let mut names = Vec::from_iter(names_set);
        names.sort_unstable();
        (names, data)
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl InventoryData {
    pub fn inventory(&self) -> JsValue {
        self.inventory.clone()
    }

    pub fn data(&self) -> JsValue {
        self.data.clone()
    }
}