/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// FunctionReplicasAffinity : Replica strategy

#[allow(unused_imports)]
use crate::models;



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FunctionReplicasAffinity {
    /// If there are > 1 replica, make sure they're on different clusters
    #[serde(rename = "cluster")]
    pub cluster: bool,
    /// If there are > 1 replica, make sure they're on different clouds
    #[serde(rename = "cloud")]
    pub cloud: bool,
}

impl FunctionReplicasAffinity {
    /// Replica strategy
    pub fn new(cluster: bool, cloud: bool) -> FunctionReplicasAffinity {
        FunctionReplicasAffinity {
            cluster,
            cloud,
        }
    }
}


