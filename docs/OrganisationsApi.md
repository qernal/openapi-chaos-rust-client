# \OrganisationsApi

All URIs are relative to *https://chaos.qernal.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**organisations_create**](OrganisationsApi.md#organisations_create) | **POST** /organisations | Create organisations
[**organisations_delete**](OrganisationsApi.md#organisations_delete) | **DELETE** /organisations/{organisation_id} | Delete an organisation
[**organisations_get**](OrganisationsApi.md#organisations_get) | **GET** /organisations/{organisation_id} | Get an organisation
[**organisations_list**](OrganisationsApi.md#organisations_list) | **GET** /organisations | List organisations
[**organisations_update**](OrganisationsApi.md#organisations_update) | **PUT** /organisations/{organisation_id} | Update an organisation



## organisations_create

> models::OrganisationResponse organisations_create(organisation_body)
Create organisations

Create an organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organisation_body** | Option<[**OrganisationBody**](OrganisationBody.md)> | Create/Update any field |  |

### Return type

[**models::OrganisationResponse**](OrganisationResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organisations_delete

> models::DeletedResponse organisations_delete(organisation_id)
Delete an organisation

Delete organisation, this will also delete all the resources within the organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organisation_id** | **uuid::Uuid** | Organisation ID reference | [required] |

### Return type

[**models::DeletedResponse**](DeletedResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organisations_get

> models::OrganisationResponse organisations_get(organisation_id)
Get an organisation

Get a single organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organisation_id** | **uuid::Uuid** | Organisation ID reference | [required] |

### Return type

[**models::OrganisationResponse**](OrganisationResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organisations_list

> models::ListOrganisationResponse organisations_list(page)
List organisations

List organisations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<[**OrganisationsListPageParameter**](.md)> | Query parameters for pagination |  |

### Return type

[**models::ListOrganisationResponse**](ListOrganisationResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organisations_update

> models::OrganisationResponse organisations_update(organisation_id, organisation_body)
Update an organisation

Update an organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organisation_id** | **uuid::Uuid** | Organisation ID reference | [required] |
**organisation_body** | Option<[**OrganisationBody**](OrganisationBody.md)> | Create/Update any field |  |

### Return type

[**models::OrganisationResponse**](OrganisationResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

