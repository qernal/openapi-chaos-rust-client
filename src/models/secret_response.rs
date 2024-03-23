/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// SecretResponse : Secret response



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecretResponse {
    /// Secret name
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: models::SecretCreateType,
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<models::SecretResponsePayload>>,
    /// Secret revision
    #[serde(rename = "revision")]
    pub revision: i32,
    #[serde(rename = "date")]
    pub date: Box<models::Date>,
}

impl SecretResponse {
    /// Secret response
    pub fn new(name: String, r#type: models::SecretCreateType, revision: i32, date: models::Date) -> SecretResponse {
        SecretResponse {
            name,
            r#type,
            payload: None,
            revision,
            date: Box::new(date),
        }
    }
}


