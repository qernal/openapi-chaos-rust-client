# \ProvidersApi

All URIs are relative to *https://chaos.qernal.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**providers_list**](ProvidersApi.md#providers_list) | **GET** /providers | Get available providers



## providers_list

> models::ListProviderResponse providers_list(page)
Get available providers

Retrieve a list of all providers with their respective deployed regions and cities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<[**OrganisationsListPageParameter**](.md)> | Query parameters for pagination |  |

### Return type

[**models::ListProviderResponse**](ListProviderResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

