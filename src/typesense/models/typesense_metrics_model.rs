use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Typesense metrics from /metrics.json endpoint
/// Uses flatten to capture all CPU metrics dynamically (system_cpu1_active_percentage through system_cpuN_active_percentage)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypesenseMetrics {
    // Aggregate CPU (always present)
    #[serde(default = "default_zero_string")]
    pub system_cpu_active_percentage: String,

    // Disk metrics
    #[serde(default = "default_zero_string")]
    pub system_disk_total_bytes: String,
    #[serde(default = "default_zero_string")]
    pub system_disk_used_bytes: String,

    // Memory metrics
    #[serde(default = "default_zero_string")]
    pub system_memory_total_bytes: String,
    #[serde(default = "default_zero_string")]
    pub system_memory_used_bytes: String,
    #[serde(default = "default_zero_string")]
    pub system_memory_total_swap_bytes: String,
    #[serde(default = "default_zero_string")]
    pub system_memory_used_swap_bytes: String,

    // Network metrics
    #[serde(default = "default_zero_string")]
    pub system_network_received_bytes: String,
    #[serde(default = "default_zero_string")]
    pub system_network_sent_bytes: String,

    // Typesense memory metrics
    #[serde(default = "default_zero_string")]
    pub typesense_memory_active_bytes: String,
    #[serde(default = "default_zero_string")]
    pub typesense_memory_allocated_bytes: String,
    #[serde(default = "default_zero_string")]
    pub typesense_memory_fragmentation_ratio: String,
    #[serde(default = "default_zero_string")]
    pub typesense_memory_mapped_bytes: String,
    #[serde(default = "default_zero_string")]
    pub typesense_memory_metadata_bytes: String,
    #[serde(default = "default_zero_string")]
    pub typesense_memory_resident_bytes: String,
    #[serde(default = "default_zero_string")]
    pub typesense_memory_retained_bytes: String,

    // Capture all other fields dynamically (including system_cpu1-N_active_percentage)
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Default for TypesenseMetrics {
    fn default() -> TypesenseMetrics {
        TypesenseMetrics {
            system_cpu_active_percentage: "0".to_string(),
            system_disk_total_bytes: "0".to_string(),
            system_disk_used_bytes: "0".to_string(),
            system_memory_total_bytes: "0".to_string(),
            system_memory_used_bytes: "0".to_string(),
            system_memory_total_swap_bytes: "0".to_string(),
            system_memory_used_swap_bytes: "0".to_string(),
            system_network_received_bytes: "0".to_string(),
            system_network_sent_bytes: "0".to_string(),
            typesense_memory_active_bytes: "0".to_string(),
            typesense_memory_allocated_bytes: "0".to_string(),
            typesense_memory_fragmentation_ratio: "0".to_string(),
            typesense_memory_mapped_bytes: "0".to_string(),
            typesense_memory_metadata_bytes: "0".to_string(),
            typesense_memory_resident_bytes: "0".to_string(),
            typesense_memory_retained_bytes: "0".to_string(),
            extra: HashMap::new(),
        }
    }
}

fn default_zero_string() -> String {
    "0".to_string()
}

impl TypesenseMetrics {
    /// Returns all CPU percentage metrics (system_cpu1_active_percentage, system_cpu2_active_percentage, etc.)
    pub fn get_cpu_percentages(&self) -> Vec<(String, f64)> {
        let mut cpus = Vec::new();

        for (key, value) in &self.extra {
            if key.starts_with("system_cpu") && key.ends_with("_active_percentage") && key != "system_cpu_active_percentage" {
                let val_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    _ => "0".to_string(),
                };
                let val: f64 = val_str.parse().unwrap_or(0.0);
                cpus.push((key.clone(), val));
            }
        }

        // Sort by CPU number for consistent output
        cpus.sort_by(|a, b| {
            let num_a = extract_cpu_number(&a.0).unwrap_or(0);
            let num_b = extract_cpu_number(&b.0).unwrap_or(0);
            num_a.cmp(&num_b)
        });

        cpus
    }
}

/// Extract the CPU number from a metric name like "system_cpu12_active_percentage"
fn extract_cpu_number(key: &str) -> Option<u32> {
    let stripped = key.strip_prefix("system_cpu")?;
    let num_str = stripped.strip_suffix("_active_percentage")?;
    num_str.parse().ok()
}
