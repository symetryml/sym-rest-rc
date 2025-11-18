use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// DataFrame structure used for learn, predict, and autoselect operations
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataFrame {
    pub attribute_names: Vec<String>,
    pub data: Vec<Vec<String>>,
    pub attribute_types: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_handling: Option<i32>,
}

/// MLContext structure used for build and autoselect operations
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MLContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attributes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attribute_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_attribute_names: Option<Vec<String>>,
    pub extra_parameters: HashMap<String, String>,
}

/// KSVSMap structure - array of key-value maps
#[derive(Serialize, Debug)]
pub struct KSVSMap {
    pub values: Vec<HashMap<String, String>>,
}

/// Helper function to parse comma-separated integers and return as strings
pub fn parse_int_list_as_strings(s: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    s.split(',')
        .map(|item| {
            // Validate it's an integer, then return as string
            item.trim()
                .parse::<i32>()
                .map(|i| i.to_string())
                .map_err(|e| format!("Failed to parse '{}' as integer: {}", item.trim(), e).into())
        })
        .collect()
}

/// Helper function to parse comma-separated strings
pub fn parse_string_list(s: &str) -> Vec<String> {
    s.split(',')
        .map(|item| item.trim().to_string())
        .collect()
}
