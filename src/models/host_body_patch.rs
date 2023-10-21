/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// HostBodyPatch : Host body update



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HostBodyPatch {
    /// Reference to secret certificate path
    #[serde(rename = "certificate", skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// If the host is disabled, then this host won't be accessible and so the deployments will not work on this host
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

impl HostBodyPatch {
    /// Host body update
    pub fn new() -> HostBodyPatch {
        HostBodyPatch {
            certificate: None,
            disabled: None,
        }
    }
}


