/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// SecretMetaResponseCertificatePayload : Secret metadata certificate payload

#[allow(unused_imports)]
use crate::models;



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecretMetaResponseCertificatePayload {
    /// Public SSL certificate
    #[serde(rename = "certificate")]
    pub certificate: String,
}

impl SecretMetaResponseCertificatePayload {
    /// Secret metadata certificate payload
    pub fn new(certificate: String) -> SecretMetaResponseCertificatePayload {
        SecretMetaResponseCertificatePayload {
            certificate,
        }
    }
}


