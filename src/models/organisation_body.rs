/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// OrganisationBody : Organisation body

#[allow(unused_imports)]
use crate::models;



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrganisationBody {
    /// Organisation name
    #[serde(rename = "name")]
    pub name: String,
}

impl OrganisationBody {
    /// Organisation body
    pub fn new(name: String) -> OrganisationBody {
        OrganisationBody {
            name,
        }
    }
}


