/*
 * Chaos
 *
 * Management API - Central user exposed system
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateProjectFunctionsRequestBodySchemaLocationAws {
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "min")]
    pub min: f32,
    #[serde(rename = "max")]
    pub max: f32,
    #[serde(rename = "scaling")]
    pub scaling: Box<crate::models::CreateProjectFunctionsRequestBodySchemaLocationAwsScaling>,
}

impl CreateProjectFunctionsRequestBodySchemaLocationAws {
    pub fn new(region: String, min: f32, max: f32, scaling: crate::models::CreateProjectFunctionsRequestBodySchemaLocationAwsScaling) -> CreateProjectFunctionsRequestBodySchemaLocationAws {
        CreateProjectFunctionsRequestBodySchemaLocationAws {
            region,
            min,
            max,
            scaling: Box::new(scaling),
        }
    }
}


