# \LoadBalancerApi

All URIs are relative to *https://api.entrywan.com/v1beta*

Method | HTTP request | Description
------------- | ------------- | -------------
[**loadbalancer_get**](LoadBalancerApi.md#loadbalancer_get) | **GET** /loadbalancer | List load balancers
[**loadbalancer_id_delete**](LoadBalancerApi.md#loadbalancer_id_delete) | **DELETE** /loadbalancer/{id} | Delete load balancer
[**loadbalancer_id_put**](LoadBalancerApi.md#loadbalancer_id_put) | **PUT** /loadbalancer/{id} | Update load balancer targets
[**loadbalancer_post**](LoadBalancerApi.md#loadbalancer_post) | **POST** /loadbalancer | Create a load balancer



## loadbalancer_get

> Vec<crate::models::LoadBalancer> loadbalancer_get()
List load balancers

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LoadBalancer>**](LoadBalancer.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## loadbalancer_id_delete

> loadbalancer_id_delete(id)
Delete load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Load balancer ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## loadbalancer_id_put

> loadbalancer_id_put(id)
Update load balancer targets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Load balancer ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## loadbalancer_post

> loadbalancer_post(loadbalancer_post_request)
Create a load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loadbalancer_post_request** | [**LoadbalancerPostRequest**](LoadbalancerPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

