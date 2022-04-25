# \ClustersApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_clusters_cluster_id**](ClustersApi.md#delete_clusters_cluster_id) | **DELETE** /clusters/{cluster-id} | Delete cluster
[**get_clusters**](ClustersApi.md#get_clusters) | **GET** /clusters | Get clusters
[**get_clusters_cluster_id**](ClustersApi.md#get_clusters_cluster_id) | **GET** /clusters/{cluster-id} | Get specific cluster
[**post_clsuters_register**](ClustersApi.md#post_clsuters_register) | **POST** /clusters | Register cluster
[**put_clusters_cluster_id**](ClustersApi.md#put_clusters_cluster_id) | **PUT** /clusters/{cluster-id} | Update cluster



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


## post_clsuters_register

> crate::models::InlineResponse200 post_clsuters_register(inline_object)
Register cluster

Register a new cluster into the system

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object** | Option<[**InlineObject**](InlineObject.md)> |  |  |

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

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

