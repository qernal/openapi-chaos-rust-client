# \DefaultApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_clusters_cluster_id**](DefaultApi.md#delete_clusters_cluster_id) | **delete** /clusters/{cluster-id} | Delete cluster
[**delete_projects_project_id**](DefaultApi.md#delete_projects_project_id) | **delete** /projects/{project-id} | Delete project
[**delete_projects_project_id_functions_function_id**](DefaultApi.md#delete_projects_project_id_functions_function_id) | **delete** /projects/{project-id}/functions/{function-id} | Delete function
[**get_clusters**](DefaultApi.md#get_clusters) | **get** /clusters | Get clusters
[**get_clusters_cluster_id**](DefaultApi.md#get_clusters_cluster_id) | **get** /clusters/{cluster-id} | Get specific cluster
[**get_projects**](DefaultApi.md#get_projects) | **get** /projects | Get projects
[**get_projects_project_id**](DefaultApi.md#get_projects_project_id) | **get** /projects/{project-id} | Get project
[**get_projects_project_id_functions**](DefaultApi.md#get_projects_project_id_functions) | **get** /projects/{project-id}/functions | Get all project functions
[**get_projects_project_id_functions_function_id**](DefaultApi.md#get_projects_project_id_functions_function_id) | **get** /projects/{project-id}/functions/{function-id} | Get function
[**get_system_health**](DefaultApi.md#get_system_health) | **get** /system/health | System Health
[**post_clsuters_register**](DefaultApi.md#post_clsuters_register) | **post** /clusters | Register cluster
[**post_deployments**](DefaultApi.md#post_deployments) | **post** /deployments | Create Deployment
[**post_projects**](DefaultApi.md#post_projects) | **post** /projects | Create project
[**put_clusters_cluster_id**](DefaultApi.md#put_clusters_cluster_id) | **put** /clusters/{cluster-id} | Update cluster
[**put_projects_project_id_functions_function_id**](DefaultApi.md#put_projects_project_id_functions_function_id) | **put** /projects/{project-id}/functions/{function-id} | Update function



## delete_clusters_cluster_id

> delete_clusters_cluster_id(cluster_id)
Delete cluster

Delete the cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_projects_project_id

> delete_projects_project_id(project_id)
Delete project

Delete a project and all of its functions

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


## get_clusters

> get_clusters()
Get clusters

Get all the clusters currently linked

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_clusters_cluster_id

> get_clusters_cluster_id(cluster_id)
Get specific cluster

Get a specific cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects

> get_projects()
Get projects

Get all the projects for that account

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_project_id

> get_projects_project_id(project_id)
Get project

Get a specific project with all its functions

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


## get_system_health

> get_system_health()
System Health

Get health of Chaos

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_clsuters_register

> post_clsuters_register()
Register cluster

Register a new cluster into the system

### Parameters

This endpoint does not need any parameter.

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
Create Deployment

Deployments can contain more than one project with many functions in - it allows you to deploy an entire API in one go rather than individual functions

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


## post_projects

> post_projects()
Create project

Create a new project

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_clusters_cluster_id

> put_clusters_cluster_id(cluster_id)
Update cluster

Update cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
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

