/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// AuthToken : API auth token

#[allow(unused_imports)]
use crate::models;



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AuthToken {
    /// Auth token uuid
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// User
    #[serde(rename = "user_id")]
    pub user_id: uuid::Uuid,
    /// Name of token
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "expiry_at", skip_serializing_if = "Option::is_none")]
    pub expiry_at: Option<String>,
    /// Combined token required for requesting an access token, this field is only returned once on creation or update (during regeneration).
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "date")]
    pub date: Box<models::Date>,
}

impl AuthToken {
    /// API auth token
    pub fn new(id: uuid::Uuid, user_id: uuid::Uuid, name: String, date: models::Date) -> AuthToken {
        AuthToken {
            id,
            user_id,
            name,
            expiry_at: None,
            token: None,
            date: Box::new(date),
        }
    }
}


