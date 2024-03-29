/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// SecretCreatePayload : Payload for secret
use super::SecretCertificate;
use super::SecretEnvironment;
use super::SecretRegistry;

#[allow(unused_imports)]
use crate::models;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecretCreatePayload {
    /// Payload for secret
    SecretCertificate(Box<SecretCertificate>),
    /// Payload for secret
    SecretEnvironment(Box<SecretEnvironment>),
    /// Payload for secret
    SecretRegistry(Box<SecretRegistry>),
}

impl Default for SecretCreatePayload {
    fn default() -> Self {
        Self::SecretCertificate(Box::new(SecretCertificate::default()))
    }
}


