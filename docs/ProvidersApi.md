# \ProvidersApi

All URIs are relative to *https://chaos.qernal.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_providers**](ProvidersApi.md#get_providers) | **GET** /providers | Get available providers



## get_providers

> Vec<serde_json::Value> get_providers()
Get available providers

Retrieve a list of all providers with their respective deployed regions and cities.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

