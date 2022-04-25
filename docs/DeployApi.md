# \DeployApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_deployments**](DeployApi.md#post_deployments) | **POST** /batch | Create Batch of Functions



## post_deployments

> post_deployments(inline_object)
Create Batch of Functions

Batch can contain more than one project with many functions in - it allows you to deploy an entire API in one go rather than individual functions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object** | Option<[**Vec<crate::models::InlineObject>**](InlineObject.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

