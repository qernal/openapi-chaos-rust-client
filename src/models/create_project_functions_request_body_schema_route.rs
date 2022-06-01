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
pub struct CreateProjectFunctionsRequestBodySchemaRoute {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "methods")]
    pub methods: Vec<String>,
}

impl CreateProjectFunctionsRequestBodySchemaRoute {
    pub fn new(path: String, methods: Vec<String>) -> CreateProjectFunctionsRequestBodySchemaRoute {
        CreateProjectFunctionsRequestBodySchemaRoute {
            path,
            methods,
        }
    }
}


