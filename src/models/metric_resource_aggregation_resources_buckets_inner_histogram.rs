/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// MetricResourceAggregationResourcesBucketsInnerHistogram : Histogram of resource usage

#[allow(unused_imports)]
use crate::models;



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetricResourceAggregationResourcesBucketsInnerHistogram {
    /// Array of resource usage by interval  > Note: A metric will have either a `counter` or `gauge` value 
    #[serde(rename = "buckets", skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<models::MetricResourceAggregationResourcesBucketsInnerHistogramBucketsInner>>,
}

impl MetricResourceAggregationResourcesBucketsInnerHistogram {
    /// Histogram of resource usage
    pub fn new() -> MetricResourceAggregationResourcesBucketsInnerHistogram {
        MetricResourceAggregationResourcesBucketsInnerHistogram {
            buckets: None,
        }
    }
}


