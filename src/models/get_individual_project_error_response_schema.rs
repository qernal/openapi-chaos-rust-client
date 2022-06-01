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
pub struct GetIndividualProjectErrorResponseSchema {
    #[serde(rename = "data")]
    pub data: String,
}

impl GetIndividualProjectErrorResponseSchema {
    pub fn new(data: String) -> GetIndividualProjectErrorResponseSchema {
        GetIndividualProjectErrorResponseSchema {
            data,
        }
    }
}


