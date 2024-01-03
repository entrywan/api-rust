# \FirewallApi

All URIs are relative to *https://api.entrywan.com/v1beta*

Method | HTTP request | Description
------------- | ------------- | -------------
[**firewall_get**](FirewallApi.md#firewall_get) | **GET** /firewall | List firewalls
[**firewall_id_delete**](FirewallApi.md#firewall_id_delete) | **DELETE** /firewall/{id} | Delete a firewall
[**firewall_post**](FirewallApi.md#firewall_post) | **POST** /firewall | Add a firewall
[**instance_id_firewall_put**](FirewallApi.md#instance_id_firewall_put) | **PUT** /instance/{id}/firewall | Apply firewall to instance



## firewall_get

> Vec<crate::models::Firewall> firewall_get()
List firewalls

List firewalls

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Firewall>**](Firewall.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## firewall_id_delete

> firewall_id_delete(id)
Delete a firewall

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of firewall | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## firewall_post

> firewall_post(firewall_post_request)
Add a firewall

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_post_request** | [**FirewallPostRequest**](FirewallPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_id_firewall_put

> instance_id_firewall_put(id)
Apply firewall to instance

Apply firewall to instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Instance ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

