# \OrganisationsApi

All URIs are relative to *https://chaos.qernal.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_organisations_org_id**](OrganisationsApi.md#delete_organisations_org_id) | **DELETE** /organisations/{org-id} | Delete organisation
[**get_organisations**](OrganisationsApi.md#get_organisations) | **GET** /organisations | Get all organisations
[**get_organisations_org_id**](OrganisationsApi.md#get_organisations_org_id) | **GET** /organisations/{org-id} | Get organisation
[**post_organisations**](OrganisationsApi.md#post_organisations) | **POST** /organisations | Create organisation
[**put_organisations_org_id**](OrganisationsApi.md#put_organisations_org_id) | **PUT** /organisations/{org-id} | Update organisation



## delete_organisations_org_id

> serde_json::Value delete_organisations_org_id(org_id)
Delete organisation

Delete an organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organisations

> Vec<crate::models::InlineResponse2002> get_organisations()
Get all organisations

Get all organisations for the logged in user

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::InlineResponse2002>**](inline_response_200_2.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organisations_org_id

> serde_json::Value get_organisations_org_id(org_id)
Get organisation

Get an organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_organisations

> serde_json::Value post_organisations(body)
Create organisation

Create a new organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_organisations_org_id

> serde_json::Value put_organisations_org_id(org_id, body)
Update organisation

Update an organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

