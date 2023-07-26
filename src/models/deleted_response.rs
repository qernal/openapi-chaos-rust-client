/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// DeletedResponse : Deleted Response



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeletedResponse {
    #[serde(rename = "message")]
    pub message: String,
}

impl DeletedResponse {
    /// Deleted Response
    pub fn new(message: String) -> DeletedResponse {
        DeletedResponse {
            message,
        }
    }
}

