/*
 * Chaos
 *
 * Management API (Central System)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BatchFunctions {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "build", skip_serializing_if = "Option::is_none")]
    pub build: Option<Box<crate::models::BatchBuild>>,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "route", skip_serializing_if = "Option::is_none")]
    pub route: Option<Box<crate::models::BatchRoute>>,
}

impl BatchFunctions {
    pub fn new(name: String, description: String, image: String) -> BatchFunctions {
        BatchFunctions {
            name,
            description,
            build: None,
            image,
            route: None,
        }
    }
}


