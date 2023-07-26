/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// ListOrganisationResponse : List organisations schema



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListOrganisationResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::PaginationMeta>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::OrganisationResponse>>,
}

impl ListOrganisationResponse {
    /// List organisations schema
    pub fn new() -> ListOrganisationResponse {
        ListOrganisationResponse {
            meta: None,
            data: None,
        }
    }
}

