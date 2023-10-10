/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// SecretBody : Secret body



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecretBody {
    /// Secret name
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: crate::models::SecretCreateType,
    #[serde(rename = "payload")]
    pub payload: Box<crate::models::SecretCreatePayload>,
    /// Encryption entity
    #[serde(rename = "encryption")]
    pub encryption: String,
}

impl SecretBody {
    /// Secret body
    pub fn new(name: String, r#type: crate::models::SecretCreateType, payload: crate::models::SecretCreatePayload, encryption: String) -> SecretBody {
        SecretBody {
            name,
            r#type,
            payload: Box::new(payload),
            encryption,
        }
    }
}

