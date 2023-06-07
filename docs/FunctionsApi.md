# \FunctionsApi

All URIs are relative to *https://chaos.qernal.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_projects_project_id_functions_function_id**](FunctionsApi.md#delete_projects_project_id_functions_function_id) | **DELETE** /projects/{project-id}/functions/{function-id} | Delete function
[**get_projects_project_id_functions**](FunctionsApi.md#get_projects_project_id_functions) | **GET** /projects/{project-id}/functions | Get all project functions
[**get_projects_project_id_functions_function_id**](FunctionsApi.md#get_projects_project_id_functions_function_id) | **GET** /projects/{project-id}/functions/{function-id} | Get function
[**post_projects_project_id_functions**](FunctionsApi.md#post_projects_project_id_functions) | **POST** /projects/{project-id}/functions | Create function
[**put_projects_project_id_functions_function_id**](FunctionsApi.md#put_projects_project_id_functions_function_id) | **PUT** /projects/{project-id}/functions/{function-id} | Update function



## delete_projects_project_id_functions_function_id

> serde_json::Value delete_projects_project_id_functions_function_id(project_id, function_id)
Delete function

Delete specific function

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**function_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_project_id_functions

> Vec<crate::models::GetProjectFunctionsSuccessResponseSchemaInner> get_projects_project_id_functions(project_id)
Get all project functions

Get the functions of a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::GetProjectFunctionsSuccessResponseSchemaInner>**](GetProjectFunctionsSuccessResponseSchema_inner.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_project_id_functions_function_id

> serde_json::Value get_projects_project_id_functions_function_id(project_id, function_id)
Get function

Get a specific function

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**function_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_projects_project_id_functions

> serde_json::Value post_projects_project_id_functions(project_id, create_project_functions_request_body_schema)
Create function

Create a function on a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**create_project_functions_request_body_schema** | Option<[**CreateProjectFunctionsRequestBodySchema**](CreateProjectFunctionsRequestBodySchema.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_projects_project_id_functions_function_id

> serde_json::Value put_projects_project_id_functions_function_id(project_id, function_id, body)
Update function

Update a specific function

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**function_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

