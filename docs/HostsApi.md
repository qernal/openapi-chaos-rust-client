# \HostsApi

All URIs are relative to *https://chaos.qernal.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**projects_hosts_create**](HostsApi.md#projects_hosts_create) | **POST** /projects/{project_id}/hosts | Create host for project
[**projects_hosts_delete**](HostsApi.md#projects_hosts_delete) | **DELETE** /projects/{project_id}/hosts/{hostname} | Delete specific host by hostname
[**projects_hosts_get**](HostsApi.md#projects_hosts_get) | **GET** /projects/{project_id}/hosts/{hostname} | Get specific host by hostname
[**projects_hosts_list**](HostsApi.md#projects_hosts_list) | **GET** /projects/{project_id}/hosts | List hosts for project
[**projects_hosts_update**](HostsApi.md#projects_hosts_update) | **PUT** /projects/{project_id}/hosts/{hostname} | Update specific host by hostname
[**projects_hosts_verify_create**](HostsApi.md#projects_hosts_verify_create) | **POST** /projects/{project_id}/hosts/{hostname}/verify | Schedule host verification task



## projects_hosts_create

> crate::models::Host projects_hosts_create(project_id, host_body)
Create host for project

Assign a host/domain to a project - hosts are globally unique and require verification, so a host cannot be assigned to multiple projects.  A host can be a valid domain, either a root domain or a subdomain. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |
**host_body** | [**HostBody**](HostBody.md) |  | [required] |

### Return type

[**crate::models::Host**](Host.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_hosts_delete

> crate::models::DeletedResponse projects_hosts_delete(project_id, hostname)
Delete specific host by hostname

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |
**hostname** | **String** | Hostname | [required] |

### Return type

[**crate::models::DeletedResponse**](DeletedResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_hosts_get

> crate::models::Host projects_hosts_get(project_id, hostname)
Get specific host by hostname

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |
**hostname** | **String** | Hostname | [required] |

### Return type

[**crate::models::Host**](Host.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_hosts_list

> crate::models::ListHosts projects_hosts_list(project_id, page)
List hosts for project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |
**page** | Option<[**OrganisationsListPageParameter**](.md)> | Query parameters for pagination |  |

### Return type

[**crate::models::ListHosts**](ListHosts.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_hosts_update

> crate::models::Host projects_hosts_update(project_id, hostname, host_body_patch)
Update specific host by hostname

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |
**hostname** | **String** | Hostname | [required] |
**host_body_patch** | [**HostBodyPatch**](HostBodyPatch.md) |  | [required] |

### Return type

[**crate::models::Host**](Host.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_hosts_verify_create

> crate::models::Host projects_hosts_verify_create(project_id, hostname)
Schedule host verification task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | Project ID reference | [required] |
**hostname** | **String** | Hostname | [required] |

### Return type

[**crate::models::Host**](Host.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

