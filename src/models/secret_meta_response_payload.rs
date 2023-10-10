/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

use super::SecretMetaResponseCertificatePayload;
use super::SecretMetaResponseDek;
use super::SecretMetaResponseRegistryPayload;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecretMetaResponsePayload {
    SecretMetaResponseCertificatePayload(Box<SecretMetaResponseCertificatePayload>),
    SecretMetaResponseDek(Box<SecretMetaResponseDek>),
    SecretMetaResponseRegistryPayload(Box<SecretMetaResponseRegistryPayload>),
}

impl Default for SecretMetaResponsePayload {
    fn default() -> Self {
        Self::SecretMetaResponseCertificatePayload(Box::new(SecretMetaResponseCertificatePayload::default()))
    }
}

