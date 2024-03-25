/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// SecretRegistry : Encrypted private container registry, `type: registry`

#[allow(unused_imports)]
use crate::models;



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecretRegistry {
    /// Url to private container repository (for docker registry use https://index.docker.io/v1/)
    #[serde(rename = "registry")]
    pub registry: String,
    /// Token used for auth to the registry
    #[serde(rename = "registry_value")]
    pub registry_value: String,
}

impl SecretRegistry {
    /// Encrypted private container registry, `type: registry`
    pub fn new(registry: String, registry_value: String) -> SecretRegistry {
        SecretRegistry {
            registry,
            registry_value,
        }
    }
}


