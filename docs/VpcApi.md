# \VpcApi

All URIs are relative to *https://api.entrywan.com/v1beta*

Method | HTTP request | Description
------------- | ------------- | -------------
[**vpc_get**](VpcApi.md#vpc_get) | **GET** /vpc | List VPC
[**vpc_id_delete**](VpcApi.md#vpc_id_delete) | **DELETE** /vpc/{id} | Delete VPC
[**vpc_id_patch**](VpcApi.md#vpc_id_patch) | **PATCH** /vpc/{id} | Remove member from VPC
[**vpc_id_post**](VpcApi.md#vpc_id_post) | **POST** /vpc/{id} | Create a VPC
[**vpc_id_put**](VpcApi.md#vpc_id_put) | **PUT** /vpc/{id} | Add member to VPC



## vpc_get

> Vec<crate::models::Vpc> vpc_get()
List VPC

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Vpc>**](VPC.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpc_id_delete

> vpc_id_delete(id)
Delete VPC

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of VPC | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpc_id_patch

> vpc_id_patch(id, vpc_id_patch_request)
Remove member from VPC

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | VPC ID | [required] |
**vpc_id_patch_request** | [**VpcIdPatchRequest**](VpcIdPatchRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpc_id_post

> vpc_id_post(id, vpc_id_post_request)
Create a VPC

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | VPC ID | [required] |
**vpc_id_post_request** | [**VpcIdPostRequest**](VpcIdPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpc_id_put

> vpc_id_put(id, vpc_id_put_request)
Add member to VPC

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | VPC ID | [required] |
**vpc_id_put_request** | [**VpcIdPutRequest**](VpcIdPutRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

