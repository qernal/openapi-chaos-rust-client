# \MetricsApi

All URIs are relative to *https://chaos.qernal.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**metrics_aggregations_list**](MetricsApi.md#metrics_aggregations_list) | **GET** /metrics/aggregations/{metric_type} | Get metrics



## metrics_aggregations_list

> models::MetricsAggregationsList200Response metrics_aggregations_list(metric_type, page, f_project, f_function, f_timestamps, f_histogram_interval)
Get metrics

Retrieve metrics for a specific project or function. Use the query parameter to request a metrics report.  > Note: Metrics are always returned in a descending order based on the timestamp. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric_type** | **String** | Metric aggregation type, types can be used with either a project or a function filter.  - httprequests: Aggregated HTTP requests - resourcestats: Aggregated resource stats (such as CPU, Memory and Network)  > Note: aggregations cannot return more than 300 data points  | [required] |
**page** | Option<[**OrganisationsListPageParameter**](.md)> | Query parameters for pagination |  |
**f_project** | Option<**uuid::Uuid**> | Project uuid reference |  |
**f_function** | Option<**uuid::Uuid**> | Function uuid reference |  |
**f_timestamps** | Option<[**LogsListFTimestampsParameter**](.md)> | Timestamp restriction for query |  |
**f_histogram_interval** | Option<**i32**> | Histogram interval |  |

### Return type

[**models::MetricsAggregationsList200Response**](metrics_aggregations_list_200_response.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

