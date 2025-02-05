/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */

/// MetricResourceAggregationResourcesBucketsInnerHistogramBucketsInnerGauge : Gauge (value at a point in time)

#[allow(unused_imports)]
use crate::models;



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MetricResourceAggregationResourcesBucketsInnerHistogramBucketsInnerGauge {
    /// Average gauge value
    #[serde(rename = "avg", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub avg: Option<Option<f64>>,
    /// Number of gauge values
    #[serde(rename = "count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub count: Option<Option<i32>>,
    /// Maximum gauge value
    #[serde(rename = "max", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max: Option<Option<f64>>,
    /// Minimum gauge value
    #[serde(rename = "min", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub min: Option<Option<f64>>,
    /// Sum of gauge values
    #[serde(rename = "sum", skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
}

impl MetricResourceAggregationResourcesBucketsInnerHistogramBucketsInnerGauge {
    /// Gauge (value at a point in time)
    pub fn new() -> MetricResourceAggregationResourcesBucketsInnerHistogramBucketsInnerGauge {
        MetricResourceAggregationResourcesBucketsInnerHistogramBucketsInnerGauge {
            avg: None,
            count: None,
            max: None,
            min: None,
            sum: None,
        }
    }
}


