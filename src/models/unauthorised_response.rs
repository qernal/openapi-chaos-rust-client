/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// UnauthorisedResponse : Unauthorised

#[allow(unused_imports)]
use crate::models;



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UnauthorisedResponse {
    #[serde(rename = "message")]
    pub message: String,
}

impl UnauthorisedResponse {
    /// Unauthorised
    pub fn new(message: String) -> UnauthorisedResponse {
        UnauthorisedResponse {
            message,
        }
    }
}


