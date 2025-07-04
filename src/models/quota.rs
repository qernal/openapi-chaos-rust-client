/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// Quota : Quota usage

#[allow(unused_imports)]
use crate::models;



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Quota {
    /// Quota name
    #[serde(rename = "name")]
    pub name: String,
    /// Quota description
    #[serde(rename = "description")]
    pub description: String,
    /// Quota limit
    #[serde(rename = "limit")]
    pub limit: i32,
    /// Quota usage
    #[serde(rename = "usage")]
    pub usage: i32,
    /// Quota type (group)
    #[serde(rename = "type")]
    pub r#type: String,
}

impl Quota {
    /// Quota usage
    pub fn new(name: String, description: String, limit: i32, usage: i32, r#type: String) -> Quota {
        Quota {
            name,
            description,
            limit,
            usage,
            r#type,
        }
    }
}


