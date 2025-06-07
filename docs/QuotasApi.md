# \QuotasApi

All URIs are relative to *https://chaos.qernal.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**organisations_quotas_get**](QuotasApi.md#organisations_quotas_get) | **GET** /organisations/{organisation_id}/quotas/{quota_entity_quota} | Get specific organisation quota
[**organisations_quotas_list**](QuotasApi.md#organisations_quotas_list) | **GET** /organisations/{organisation_id}/quotas | List organisation quotas
[**projects_quotas_get**](QuotasApi.md#projects_quotas_get) | **GET** /projects/{project_id}/quotas/{quota_entity_quota} | Get specific project quota
[**projects_quotas_list**](QuotasApi.md#projects_quotas_list) | **GET** /projects/{project_id}/quotas | List project quotas
[**users_quotas_get**](QuotasApi.md#users_quotas_get) | **GET** /users/{user_id}/quotas/{quota_entity_quota} | Get specific user quota
[**users_quotas_list**](QuotasApi.md#users_quotas_list) | **GET** /users/{user_id}/quotas | List user quotas



## organisations_quotas_get

> Vec<models::Quota> organisations_quotas_get(organisation_id, quota_entity_quota)
Get specific organisation quota

Get a specific quota for an organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organisation_id** | **uuid::Uuid** | Organisation ID reference | [required] |
**quota_entity_quota** | **String** |  | [required] |

### Return type

[**Vec<models::Quota>**](Quota.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organisations_quotas_list

> Vec<models::Quota> organisations_quotas_list(organisation_id)
List organisation quotas

Get the quotas for an organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organisation_id** | **uuid::Uuid** | Organisation ID reference | [required] |

### Return type

[**Vec<models::Quota>**](Quota.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_quotas_get

> Vec<models::Quota> projects_quotas_get(project_id, quota_entity_quota)
Get specific project quota

Get a specific quota for a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |
**quota_entity_quota** | **String** |  | [required] |

### Return type

[**Vec<models::Quota>**](Quota.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_quotas_list

> Vec<models::Quota> projects_quotas_list(project_id)
List project quotas

Get the quotas for a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |

### Return type

[**Vec<models::Quota>**](Quota.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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

