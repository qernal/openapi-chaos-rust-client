# \OrganisationsApi

All URIs are relative to *https://chaos.qernal.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_organisations_org_id**](OrganisationsApi.md#delete_organisations_org_id) | **DELETE** /organisations/{organisation_id} | Delete an organisation
[**get_organisations**](OrganisationsApi.md#get_organisations) | **GET** /organisations | List organisations
[**get_organisations_org_id**](OrganisationsApi.md#get_organisations_org_id) | **GET** /organisations/{organisation_id} | Get an organisation
[**post_organisations**](OrganisationsApi.md#post_organisations) | **POST** /organisations | Create organisations
[**put_organisations_org_id**](OrganisationsApi.md#put_organisations_org_id) | **PUT** /organisations/{organisation_id} | Update an organisation



## delete_organisations_org_id

> crate::models::DeletedResponse delete_organisations_org_id(organisation_id)
Delete an organisation

Delete organisation, this will also delete all the resources within the organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organisation_id** | **uuid::Uuid** | Organisation ID reference | [required] |

### Return type

[**crate::models::DeletedResponse**](DeletedResponse.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organisations

> crate::models::ListOrganisationResponse get_organisations(page)
List organisations

List organisations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<[**GetOrganisationsPageParameter**](.md)> | Query parameters for pagination |  |

### Return type

[**crate::models::ListOrganisationResponse**](ListOrganisationResponse.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organisations_org_id

> crate::models::OrganisationResponse get_organisations_org_id(organisation_id)
Get an organisation

Get a single organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organisation_id** | **uuid::Uuid** | Organisation ID reference | [required] |

### Return type

[**crate::models::OrganisationResponse**](OrganisationResponse.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_organisations

> crate::models::OrganisationResponse post_organisations(organisation_body)
Create organisations

Create an organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organisation_body** | Option<[**OrganisationBody**](OrganisationBody.md)> | Create/Update any field |  |

### Return type

[**crate::models::OrganisationResponse**](OrganisationResponse.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_organisations_org_id

> crate::models::OrganisationResponse put_organisations_org_id(organisation_id, organisation_body)
Update an organisation

Update an organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organisation_id** | **uuid::Uuid** | Organisation ID reference | [required] |
**organisation_body** | Option<[**OrganisationBody**](OrganisationBody.md)> | Create/Update any field |  |

### Return type

[**crate::models::OrganisationResponse**](OrganisationResponse.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

