# \FunctionsApi

All URIs are relative to *https://chaos.qernal.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**functions_create**](FunctionsApi.md#functions_create) | **POST** /functions | Create function
[**functions_delete**](FunctionsApi.md#functions_delete) | **DELETE** /functions/{function_id} | Delete function
[**functions_get**](FunctionsApi.md#functions_get) | **GET** /functions/{function_id} | Get function (latest revision)
[**functions_revisions_get**](FunctionsApi.md#functions_revisions_get) | **GET** /functions/{function_id}/revisions/{function_revision_id} | Get a specific revision of a function
[**functions_revisions_list**](FunctionsApi.md#functions_revisions_list) | **GET** /functions/{function_id}/revisions | List all revisions for a function
[**functions_update**](FunctionsApi.md#functions_update) | **PUT** /functions/{function_id} | Update function
[**projects_functions_list**](FunctionsApi.md#projects_functions_list) | **GET** /projects/{project_id}/functions | List all functions within a project



## functions_create

> models::Function functions_create(function_body)
Create function

Create a new function

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_body** | [**FunctionBody**](FunctionBody.md) | Create/Update any field | [required] |

### Return type

[**models::Function**](Function.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_delete

> models::DeletedResponse functions_delete(function_id)
Delete function

Delete a function (and all revisions)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **uuid::Uuid** | Function ID reference | [required] |

### Return type

[**models::DeletedResponse**](DeletedResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_get

> models::Function functions_get(function_id)
Get function (latest revision)

Get a specific function (latest revision)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **uuid::Uuid** | Function ID reference | [required] |

### Return type

[**models::Function**](Function.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_revisions_get

> models::Function functions_revisions_get(function_id, function_revision_id)
Get a specific revision of a function

Get a specific revision of a function

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **uuid::Uuid** | Function ID reference | [required] |
**function_revision_id** | **uuid::Uuid** | Function revision ID reference | [required] |

### Return type

[**models::Function**](Function.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_revisions_list

> models::ListFunction functions_revisions_list(function_id, page)
List all revisions for a function

List all revisions for a function

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **uuid::Uuid** | Function ID reference | [required] |
**page** | Option<[**OrganisationsListPageParameter**](.md)> | Query parameters for pagination |  |

### Return type

[**models::ListFunction**](ListFunction.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_update

> models::Function functions_update(function_id, function)
Update function

Update a function (creates a new revision)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_id** | **uuid::Uuid** | Function ID reference | [required] |
**function** | [**Function**](Function.md) | Update any field | [required] |

### Return type

[**models::Function**](Function.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_functions_list

> models::ListFunction projects_functions_list(project_id, page)
List all functions within a project

List all functions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |
**page** | Option<[**OrganisationsListPageParameter**](.md)> | Query parameters for pagination |  |

### Return type

[**models::ListFunction**](ListFunction.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

