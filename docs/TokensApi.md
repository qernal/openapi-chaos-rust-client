# \TokensApi

All URIs are relative to *https://chaos.qernal.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_tokens_create**](TokensApi.md#auth_tokens_create) | **POST** /auth/tokens | Create new auth token
[**auth_tokens_delete**](TokensApi.md#auth_tokens_delete) | **DELETE** /auth/tokens/{token_id} | Delete token
[**auth_tokens_get**](TokensApi.md#auth_tokens_get) | **GET** /auth/tokens/{token_id} | Get token information
[**auth_tokens_list**](TokensApi.md#auth_tokens_list) | **GET** /auth/tokens | List all user auth tokens
[**auth_tokens_update**](TokensApi.md#auth_tokens_update) | **PUT** /auth/tokens/{token_id} | Update token



## auth_tokens_create

> models::AuthToken auth_tokens_create(auth_token_body)
Create new auth token

Create new auth token for use with the CLI and TF Provider  ### Warning The `token` field is only shown once and can't be retrieved again without generating a new token. Securely save this once the response has been received. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_token_body** | [**AuthTokenBody**](AuthTokenBody.md) |  | [required] |

### Return type

[**models::AuthToken**](AuthToken.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_tokens_delete

> models::DeletedResponse auth_tokens_delete(token_id)
Delete token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **uuid::Uuid** | Token ID reference | [required] |

### Return type

[**models::DeletedResponse**](DeletedResponse.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_tokens_get

> models::AuthTokenMeta auth_tokens_get(token_id)
Get token information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **uuid::Uuid** | Token ID reference | [required] |

### Return type

[**models::AuthTokenMeta**](AuthTokenMeta.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_tokens_list

> models::ListAuthTokens auth_tokens_list(page)
List all user auth tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<[**OrganisationsListPageParameter**](.md)> | Query parameters for pagination |  |

### Return type

[**models::ListAuthTokens**](ListAuthTokens.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_tokens_update

> models::AuthToken auth_tokens_update(token_id, auth_token_patch)
Update token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **uuid::Uuid** | Token ID reference | [required] |
**auth_token_patch** | [**AuthTokenPatch**](AuthTokenPatch.md) |  | [required] |

### Return type

[**models::AuthToken**](AuthToken.md)

### Authorization

[cookie](../README.md#cookie), [token](../README.md#token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

