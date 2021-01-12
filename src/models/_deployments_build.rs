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
pub struct DeploymentsBuild {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "dockerfile")]
    pub dockerfile: String,
    #[serde(rename = "arch")]
    pub arch: Vec<serde_json::Value>,
}

impl DeploymentsBuild {
    pub fn new(path: String, dockerfile: String, arch: Vec<serde_json::Value>) -> DeploymentsBuild {
        DeploymentsBuild {
            path,
            dockerfile,
            arch,
        }
    }
}


