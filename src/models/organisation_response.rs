/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// OrganisationResponse : Organisation response



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrganisationResponse {
    /// Organisation id
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// User id
    #[serde(rename = "user_id")]
    pub user_id: uuid::Uuid,
    /// Organisation name
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "date")]
    pub date: Box<models::Date>,
}

impl OrganisationResponse {
    /// Organisation response
    pub fn new(id: uuid::Uuid, user_id: uuid::Uuid, name: String, date: models::Date) -> OrganisationResponse {
        OrganisationResponse {
            id,
            user_id,
            name,
            date: Box::new(date),
        }
    }
}


