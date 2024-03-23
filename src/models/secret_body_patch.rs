/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// SecretBodyPatch : Secret body patch fields



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecretBodyPatch {
    #[serde(rename = "type")]
    pub r#type: models::SecretCreateType,
    #[serde(rename = "payload")]
    pub payload: Box<models::SecretCreatePayload>,
    /// Encryption entity
    #[serde(rename = "encryption")]
    pub encryption: String,
}

impl SecretBodyPatch {
    /// Secret body patch fields
    pub fn new(r#type: models::SecretCreateType, payload: models::SecretCreatePayload, encryption: String) -> SecretBodyPatch {
        SecretBodyPatch {
            r#type,
            payload: Box::new(payload),
            encryption,
        }
    }
}


