/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// ListProjectResponse : List of projects



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListProjectResponse {
    #[serde(rename = "meta")]
    pub meta: Box<models::PaginationMeta>,
    #[serde(rename = "data")]
    pub data: Vec<models::ProjectResponse>,
}

impl ListProjectResponse {
    /// List of projects
    pub fn new(meta: models::PaginationMeta, data: Vec<models::ProjectResponse>) -> ListProjectResponse {
        ListProjectResponse {
            meta: Box::new(meta),
            data,
        }
    }
}


