use serde::{Deserialize, Serialize};
use serde_json::Map;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TypesenseStats {
    #[serde(default)]
    pub cache_hit_ratio: f64,
    #[serde(default)]
    pub delete_latency_ms: f64,
    #[serde(default)]
    pub delete_requests_per_second: f64,
    #[serde(default)]
    pub import_latency_ms: f64,
    #[serde(default)]
    pub import_requests_per_second: f64,
    #[serde(default)]
    pub latency_ms: Map<String, Value>,
    #[serde(default)]
    pub overloaded_requests_per_second: f64,
    #[serde(default)]
    pub pending_write_batches: f64,
    #[serde(default)]
    pub requests_per_second: Map<String, Value>,
    #[serde(default)]
    pub search_latency_ms: f64,
    #[serde(default)]
    pub search_requests_per_second: f64,
    #[serde(default)]
    pub total_requests_per_second: f64,
    #[serde(default)]
    pub write_latency_ms: f64,
    #[serde(default)]
    pub write_requests_per_second: f64,
}
