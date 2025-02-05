/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// ListLogResponse : List of log

#[allow(unused_imports)]
use crate::models;



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListLogResponse {
    #[serde(rename = "meta")]
    pub meta: Box<models::PaginationMeta>,
    #[serde(rename = "data")]
    pub data: Vec<models::Log>,
}

impl ListLogResponse {
    /// List of log
    pub fn new(meta: models::PaginationMeta, data: Vec<models::Log>) -> ListLogResponse {
        ListLogResponse {
            meta: Box::new(meta),
            data,
        }
    }
}


