/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// SecretMetaResponseDek : DEK secret, `type: dek`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecretMetaResponseDek {
    /// Base64 encoded DEK public key
    #[serde(rename = "public")]
    pub public: String,
}

impl SecretMetaResponseDek {
    /// DEK secret, `type: dek`
    pub fn new(public: String) -> SecretMetaResponseDek {
        SecretMetaResponseDek {
            public,
        }
    }
}


