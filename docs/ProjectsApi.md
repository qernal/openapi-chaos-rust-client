# \ProjectsApi

All URIs are relative to *https://chaos.qernal.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**organisations_projects_list**](ProjectsApi.md#organisations_projects_list) | **GET** /organisations/{organisation_id}/projects | Get all projects within an organisation
[**projects_create**](ProjectsApi.md#projects_create) | **POST** /projects | Create project
[**projects_delete**](ProjectsApi.md#projects_delete) | **DELETE** /projects/{project_id} | Delete project
[**projects_get**](ProjectsApi.md#projects_get) | **GET** /projects/{project_id} | Get project
[**projects_list**](ProjectsApi.md#projects_list) | **GET** /projects | List projects
[**projects_quotas_get**](ProjectsApi.md#projects_quotas_get) | **GET** /projects/{project_id}/quotas/{quota_entity_quota} | Get specific project quota
[**projects_quotas_list**](ProjectsApi.md#projects_quotas_list) | **GET** /projects/{project_id}/quotas | List project quotas
[**projects_update**](ProjectsApi.md#projects_update) | **PUT** /projects/{project_id} | Update project



## organisations_projects_list

> models::ListProjectResponse organisations_projects_list(organisation_id, page, f_name)
Get all projects within an organisation

Get all the projects linked to a specific organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organisation_id** | **uuid::Uuid** | Organisation ID reference | [required] |
**page** | Option<[**OrganisationsListPageParameter**](.md)> | Query parameters for pagination |  |
**f_name** | Option<**String**> | Filter resource on name, if the value ends in an asterix it will be treated as a partial search otherwise, it'll be an exact match  |  |

### Return type

[**models::ListProjectResponse**](ListProjectResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_create

> models::ProjectResponse projects_create(project_body)
Create project

Create a new project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_body** | Option<[**ProjectBody**](ProjectBody.md)> | Create/Update any field |  |

### Return type

[**models::ProjectResponse**](ProjectResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_delete

> models::DeletedResponse projects_delete(project_id)
Delete project

Delete project, this will also delete all the resources within the project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |

### Return type

[**models::DeletedResponse**](DeletedResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_get

> models::ProjectResponse projects_get(project_id)
Get project

Get a specific project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |

### Return type

[**models::ProjectResponse**](ProjectResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_list

> models::ListProjectResponse projects_list(page, f_name)
List projects

Get all projects for this user, paginated

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<[**OrganisationsListPageParameter**](.md)> | Query parameters for pagination |  |
**f_name** | Option<**String**> | Filter resource on name, if the value ends in an asterix it will be treated as a partial search otherwise, it'll be an exact match  |  |

### Return type

[**models::ListProjectResponse**](ListProjectResponse.md)

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


## projects_update

> models::ProjectResponse projects_update(project_id, project_body_patch)
Update project

Update project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |
**project_body_patch** | Option<[**ProjectBodyPatch**](ProjectBodyPatch.md)> | Update any field |  |

### Return type

[**models::ProjectResponse**](ProjectResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

