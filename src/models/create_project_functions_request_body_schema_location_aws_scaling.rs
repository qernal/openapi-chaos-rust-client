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
pub struct CreateProjectFunctionsRequestBodySchemaLocationAwsScaling {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "average")]
    pub average: f32,
}

impl CreateProjectFunctionsRequestBodySchemaLocationAwsScaling {
    pub fn new(_type: String, average: f32) -> CreateProjectFunctionsRequestBodySchemaLocationAwsScaling {
        CreateProjectFunctionsRequestBodySchemaLocationAwsScaling {
            _type,
            average,
        }
    }
}


