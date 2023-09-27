/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// SecretEnvironment : Encrypted ENV secret, `type: environment`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecretEnvironment {
    /// Encrypted environment pairs (key - env key, value - env value)
    #[serde(rename = "environment_value")]
    pub environment_value: String,
}

impl SecretEnvironment {
    /// Encrypted ENV secret, `type: environment`
    pub fn new(environment_value: String) -> SecretEnvironment {
        SecretEnvironment {
            environment_value,
        }
    }
}


