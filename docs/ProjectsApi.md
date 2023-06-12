# \ProjectsApi

All URIs are relative to *https://chaos.qernal.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_projects_project_id**](ProjectsApi.md#delete_projects_project_id) | **DELETE** /projects/{project_id} | Delete project
[**get_organisations_org_id_projects**](ProjectsApi.md#get_organisations_org_id_projects) | **GET** /organisations/{organisation_id}/projects | Get all projects within an organisation
[**get_projects**](ProjectsApi.md#get_projects) | **GET** /projects | List projects
[**get_projects_project_id**](ProjectsApi.md#get_projects_project_id) | **GET** /projects/{project_id} | Get project
[**post_projects**](ProjectsApi.md#post_projects) | **POST** /projects | Create project
[**put_projects_project_id**](ProjectsApi.md#put_projects_project_id) | **PUT** /projects/{project_id} | Update project



## delete_projects_project_id

> crate::models::DeletedResponse delete_projects_project_id(project_id)
Delete project

Delete project, this will also delete all the resources within the project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |

### Return type

[**crate::models::DeletedResponse**](DeletedResponse.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organisations_org_id_projects

> crate::models::ListProjectResponse get_organisations_org_id_projects(organisation_id, page)
Get all projects within an organisation

Get all the projects linked to a specific organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organisation_id** | **uuid::Uuid** | Organisation ID reference | [required] |
**page** | Option<[**GetOrganisationsPageParameter**](.md)> | Query parameters for pagination |  |

### Return type

[**crate::models::ListProjectResponse**](ListProjectResponse.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects

> crate::models::ListProjectResponse get_projects(page)
List projects

Get all projects for this user, paginated

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<[**GetOrganisationsPageParameter**](.md)> | Query parameters for pagination |  |

### Return type

[**crate::models::ListProjectResponse**](ListProjectResponse.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_project_id

> crate::models::ProjectResponse get_projects_project_id(project_id)
Get project

Get a specific project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |

### Return type

[**crate::models::ProjectResponse**](ProjectResponse.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_projects

> crate::models::ProjectResponse post_projects(project_body)
Create project

Create a new project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_body** | Option<[**ProjectBody**](ProjectBody.md)> | Create/Update any field |  |

### Return type

[**crate::models::ProjectResponse**](ProjectResponse.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_projects_project_id

> crate::models::ProjectResponse put_projects_project_id(project_id, project_body)
Update project

Update project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |
**project_body** | Option<[**ProjectBody**](ProjectBody.md)> | Create/Update any field |  |

### Return type

[**crate::models::ProjectResponse**](ProjectResponse.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

