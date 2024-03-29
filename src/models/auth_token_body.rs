/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// AuthTokenBody : API auth token create

#[allow(unused_imports)]
use crate::models;



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AuthTokenBody {
    /// Name of token
    #[serde(rename = "name")]
    pub name: String,
    /// Token expiration duration in days. 0 - token will never expire
    #[serde(rename = "expiry_duration")]
    pub expiry_duration: i32,
}

impl AuthTokenBody {
    /// API auth token create
    pub fn new(name: String, expiry_duration: i32) -> AuthTokenBody {
        AuthTokenBody {
            name,
            expiry_duration,
        }
    }
}


