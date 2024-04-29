/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// ProviderInnerLocations : Locations at varying levels this provider operates within

#[allow(unused_imports)]
use crate::models;



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProviderInnerLocations {
    #[serde(rename = "continents", skip_serializing_if = "Option::is_none")]
    pub continents: Option<Vec<String>>,
    #[serde(rename = "countries", skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<String>>,
    #[serde(rename = "cities", skip_serializing_if = "Option::is_none")]
    pub cities: Option<Vec<String>>,
}

impl ProviderInnerLocations {
    /// Locations at varying levels this provider operates within
    pub fn new() -> ProviderInnerLocations {
        ProviderInnerLocations {
            continents: None,
            countries: None,
            cities: None,
        }
    }
}

