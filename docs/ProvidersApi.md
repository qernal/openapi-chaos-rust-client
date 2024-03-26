# \ProvidersApi

All URIs are relative to *https://chaos.qernal.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**providers_get**](ProvidersApi.md#providers_get) | **GET** /providers | Get available providers



## providers_get

> Vec<models::ProviderInner> providers_get()
Get available providers

Retrieve a list of all providers with their respective deployed regions and cities.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ProviderInner>**](Provider_inner.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

