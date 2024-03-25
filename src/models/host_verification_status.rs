/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// HostVerificationStatus : Host verification status

#[allow(unused_imports)]
use crate::models;

/// Host verification status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HostVerificationStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,

}

impl ToString for HostVerificationStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Pending => String::from("pending"),
            Self::Completed => String::from("completed"),
            Self::Failed => String::from("failed"),
        }
    }
}

impl Default for HostVerificationStatus {
    fn default() -> HostVerificationStatus {
        Self::Pending
    }
}




