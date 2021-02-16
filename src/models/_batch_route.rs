/*
 * Chaos
 *
 * Management API (Central System)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchRoute {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "methods")]
    pub methods: Vec<serde_json::Value>,
}

impl BatchRoute {
    pub fn new(path: String, methods: Vec<serde_json::Value>) -> BatchRoute {
        BatchRoute {
            path,
            methods,
        }
    }
}


