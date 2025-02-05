# \LogsApi

All URIs are relative to *https://chaos.qernal.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**logs_list**](LogsApi.md#logs_list) | **GET** /logs | Get logs



## logs_list

> models::ListLogResponse logs_list(page, f_project, f_function, f_timestamps, f_query, f_log_type)
Get logs

Retrieve logs for a specific project or function. Use the query parameter to search logs.  > Note: Logs are always returned in a descending order based on the timestamp. > Note: A max size of 500 logs is returned per request (when using page[size]). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<[**OrganisationsListPageParameter**](.md)> | Query parameters for pagination |  |
**f_project** | Option<**uuid::Uuid**> | Project uuid reference |  |
**f_function** | Option<**uuid::Uuid**> | Function uuid reference |  |
**f_timestamps** | Option<[**LogsListFTimestampsParameter**](.md)> | Timestamp restriction for query |  |
**f_query** | Option<**String**> | Text query string |  |
**f_log_type** | Option<**String**> | Type of log |  |

### Return type

[**models::ListLogResponse**](ListLogResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

