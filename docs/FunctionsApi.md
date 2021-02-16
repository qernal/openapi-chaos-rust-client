# \FunctionsApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_projects_project_id_functions_function_id**](FunctionsApi.md#delete_projects_project_id_functions_function_id) | **delete** /projects/{project-id}/functions/{function-id} | Delete function
[**get_projects_project_id_functions**](FunctionsApi.md#get_projects_project_id_functions) | **get** /projects/{project-id}/functions | Get all project functions
[**get_projects_project_id_functions_function_id**](FunctionsApi.md#get_projects_project_id_functions_function_id) | **get** /projects/{project-id}/functions/{function-id} | Get function
[**post_deployments**](FunctionsApi.md#post_deployments) | **post** /batch | Create Batch of Functions
[**put_projects_project_id_functions_function_id**](FunctionsApi.md#put_projects_project_id_functions_function_id) | **put** /projects/{project-id}/functions/{function-id} | Update function



## delete_projects_project_id_functions_function_id

> delete_projects_project_id_functions_function_id(project_id, function_id)
Delete function

Delete specific function

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**function_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_project_id_functions

> get_projects_project_id_functions(project_id)
Get all project functions

Get the functions of a project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_project_id_functions_function_id

> get_projects_project_id_functions_function_id(project_id, function_id)
Get function

Get a specific function

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**function_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## put_projects_project_id_functions_function_id

> put_projects_project_id_functions_function_id(project_id, function_id)
Update function

Update a specific function

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |
**function_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

