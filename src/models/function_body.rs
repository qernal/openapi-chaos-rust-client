/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// FunctionBody : Function create body

#[allow(unused_imports)]
use crate::models;



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FunctionBody {
    /// ID of the project this function belongs to
    #[serde(rename = "project_id")]
    pub project_id: uuid::Uuid,
    /// Function spec version
    #[serde(rename = "version")]
    pub version: Version,
    /// Name of the function
    #[serde(rename = "name")]
    pub name: String,
    /// Description of what the function does
    #[serde(rename = "description")]
    pub description: String,
    /// Path to container image
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "type")]
    pub r#type: models::FunctionType,
    #[serde(rename = "size")]
    pub size: Box<models::FunctionSize>,
    /// Port the application runs on
    #[serde(rename = "port")]
    pub port: i32,
    /// The public route/path to this function, only applicable to http type functions
    #[serde(rename = "routes", skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<models::FunctionRoute>>,
    #[serde(rename = "scaling")]
    pub scaling: Box<models::FunctionScaling>,
    /// List of deployments for this function
    #[serde(rename = "deployments")]
    pub deployments: Vec<models::FunctionDeploymentBody>,
    /// List of environment variables for secrets
    #[serde(rename = "secrets")]
    pub secrets: Vec<models::FunctionEnv>,
    /// Tags to limit deployment
    #[serde(rename = "compliance", skip_serializing_if = "Option::is_none")]
    pub compliance: Option<Vec<models::FunctionCompliance>>,
}

impl FunctionBody {
    /// Function create body
    pub fn new(project_id: uuid::Uuid, version: Version, name: String, description: String, image: String, r#type: models::FunctionType, size: models::FunctionSize, port: i32, scaling: models::FunctionScaling, deployments: Vec<models::FunctionDeploymentBody>, secrets: Vec<models::FunctionEnv>) -> FunctionBody {
        FunctionBody {
            project_id,
            version,
            name,
            description,
            image,
            r#type,
            size: Box::new(size),
            port,
            routes: None,
            scaling: Box::new(scaling),
            deployments,
            secrets,
            compliance: None,
        }
    }
}

/// Function spec version
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "1.0.0")]
    Variant1Period0Period0,
}

impl Default for Version {
    fn default() -> Version {
        Self::Variant1Period0Period0
    }
}

