# \ProjectsApi

All URIs are relative to *https://chaos.qernal.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_projects_project_id**](ProjectsApi.md#delete_projects_project_id) | **DELETE** /projects/{project-id} | Delete project
[**get_projects**](ProjectsApi.md#get_projects) | **GET** /projects | Get projects
[**get_projects_project_id**](ProjectsApi.md#get_projects_project_id) | **GET** /projects/{project-id} | Get project
[**post_projects**](ProjectsApi.md#post_projects) | **POST** /projects | Create project



## delete_projects_project_id

> serde_json::Value delete_projects_project_id(project_id)
Delete project

Delete a project and all of its functions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects

> Vec<crate::models::InlineResponse2001> get_projects()
Get projects

Get all the projects for that account

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::InlineResponse2001>**](inline_response_200_1.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_project_id

> serde_json::Value get_projects_project_id(project_id)
Get project

Get a specific project with all its functions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_projects

> serde_json::Value post_projects(create_project_request_body_schema)
Create project

Create a new project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_project_request_body_schema** | Option<[**CreateProjectRequestBodySchema**](CreateProjectRequestBodySchema.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

