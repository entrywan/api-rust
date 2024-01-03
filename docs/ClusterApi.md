# \ClusterApi

All URIs are relative to *https://api.entrywan.com/v1beta*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cluster_get**](ClusterApi.md#cluster_get) | **GET** /cluster | List kubernetes clusters
[**cluster_id_delete**](ClusterApi.md#cluster_id_delete) | **DELETE** /cluster/{id} | Delete cluster
[**cluster_id_get**](ClusterApi.md#cluster_id_get) | **GET** /cluster/{id} | Get a kubernetes cluster
[**cluster_id_kubeconfig_get**](ClusterApi.md#cluster_id_kubeconfig_get) | **GET** /cluster/{id}/kubeconfig | Fetch the kubeconfig file for a cluster
[**cluster_id_rotate_patch**](ClusterApi.md#cluster_id_rotate_patch) | **PATCH** /cluster/{id}/rotate | Rotate credentials
[**cluster_id_scale_put**](ClusterApi.md#cluster_id_scale_put) | **PUT** /cluster/{id}/scale | Scale a cluster up or down
[**cluster_post**](ClusterApi.md#cluster_post) | **POST** /cluster | Create a kubernetes cluster



## cluster_get

> Vec<crate::models::Cluster> cluster_get()
List kubernetes clusters

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Cluster>**](Cluster.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_id_delete

> cluster_id_delete(id)
Delete cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_id_get

> crate::models::Cluster cluster_id_get(id)
Get a kubernetes cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

[**crate::models::Cluster**](Cluster.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_id_kubeconfig_get

> String cluster_id_kubeconfig_get(id)
Fetch the kubeconfig file for a cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

**String**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_id_rotate_patch

> cluster_id_rotate_patch(id)
Rotate credentials

Rotate certificates for the cluster.  This invalidates the kubeconfig secrets generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Cluster ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_id_scale_put

> cluster_id_scale_put(id, cluster_id_scale_put_request)
Scale a cluster up or down

Scale the number of worker nodes in a cluster up or down

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Cluster ID | [required] |
**cluster_id_scale_put_request** | [**ClusterIdScalePutRequest**](ClusterIdScalePutRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_post

> cluster_post(cluster_post_request)
Create a kubernetes cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_post_request** | [**ClusterPostRequest**](ClusterPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

