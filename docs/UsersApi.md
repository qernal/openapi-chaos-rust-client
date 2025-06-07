# \UsersApi

All URIs are relative to *https://chaos.qernal.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_quotas_get**](UsersApi.md#users_quotas_get) | **GET** /users/{user_id}/quotas/{quota_entity_quota} | Get specific user quota
[**users_quotas_list**](UsersApi.md#users_quotas_list) | **GET** /users/{user_id}/quotas | List user quotas



## users_quotas_get

> Vec<models::Quota> users_quotas_get(user_id, quota_entity_quota)
Get specific user quota

Get a specific quota for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | User ID reference | [required] |
**quota_entity_quota** | **String** |  | [required] |

### Return type

[**Vec<models::Quota>**](Quota.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_quotas_list

> Vec<models::Quota> users_quotas_list(user_id)
List user quotas

Get the quotas for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | User ID reference | [required] |

### Return type

[**Vec<models::Quota>**](Quota.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

