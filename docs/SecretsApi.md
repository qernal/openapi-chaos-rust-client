# \SecretsApi

All URIs are relative to *https://chaos.qernal.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**projects_secrets_create**](SecretsApi.md#projects_secrets_create) | **POST** /projects/{project_id}/secrets | Create project secret
[**projects_secrets_delete**](SecretsApi.md#projects_secrets_delete) | **DELETE** /projects/{project_id}/secrets/{secret_name} | Delete project secret
[**projects_secrets_get**](SecretsApi.md#projects_secrets_get) | **GET** /projects/{project_id}/secrets/{secret_name} | Get project secret
[**projects_secrets_list**](SecretsApi.md#projects_secrets_list) | **GET** /projects/{project_id}/secrets | List project secrets of a specific type
[**projects_secrets_update**](SecretsApi.md#projects_secrets_update) | **PUT** /projects/{project_id}/secrets/{secret_name} | Update project secret



## projects_secrets_create

> models::SecretResponse projects_secrets_create(project_id, secret_body)
Create project secret

Create a new project secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |
**secret_body** | [**SecretBody**](SecretBody.md) | Create/Update any field  The example generated may only be for one of the secret types, look towards the payload section of the schema for further fields, values and examples.  | [required] |

### Return type

[**models::SecretResponse**](SecretResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_secrets_delete

> models::DeletedResponse projects_secrets_delete(project_id, secret_name)
Delete project secret

Delete project secret, if the secret is still linked to an active/deployed function - it cannot be removed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |
**secret_name** | **String** | Unique secret name | [required] |

### Return type

[**models::DeletedResponse**](DeletedResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_secrets_get

> models::SecretMetaResponse projects_secrets_get(project_id, secret_name)
Get project secret

Get a specific project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |
**secret_name** | **String** | Unique secret name | [required] |

### Return type

[**models::SecretMetaResponse**](SecretMetaResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_secrets_list

> models::ListSecretResponse projects_secrets_list(project_id, page, secret_type)
List project secrets of a specific type

List project secrets of a specific type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |
**page** | Option<[**OrganisationsListPageParameter**](.md)> | Query parameters for pagination |  |
**secret_type** | Option<[**SecretMetaType**](.md)> | Type of secret to filter on |  |

### Return type

[**models::ListSecretResponse**](ListSecretResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_secrets_update

> models::SecretResponse projects_secrets_update(project_id, secret_name, secret_body_patch)
Update project secret

Update project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |
**secret_name** | **String** | Unique secret name | [required] |
**secret_body_patch** | [**SecretBodyPatch**](SecretBodyPatch.md) | Update any field | [required] |

### Return type

[**models::SecretResponse**](SecretResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

