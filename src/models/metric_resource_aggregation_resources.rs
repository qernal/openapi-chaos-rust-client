/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// MetricResourceAggregationResources : Resource(s) aggregation

#[allow(unused_imports)]
use crate::models;



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetricResourceAggregationResources {
    /// Array of unqiue resources
    #[serde(rename = "buckets", skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<models::MetricResourceAggregationResourcesBucketsInner>>,
    /// Upper bound of error in document count
    #[serde(rename = "doc_count_error_upper_bound", skip_serializing_if = "Option::is_none")]
    pub doc_count_error_upper_bound: Option<i32>,
    /// Sum of other document counts
    #[serde(rename = "sum_other_doc_count", skip_serializing_if = "Option::is_none")]
    pub sum_other_doc_count: Option<i32>,
}

impl MetricResourceAggregationResources {
    /// Resource(s) aggregation
    pub fn new() -> MetricResourceAggregationResources {
        MetricResourceAggregationResources {
            buckets: None,
            doc_count_error_upper_bound: None,
            sum_other_doc_count: None,
        }
    }
}


