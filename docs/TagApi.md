# \TagApi

All URIs are relative to *https://api.entrywan.com/v1beta*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tag_id_get**](TagApi.md#tag_id_get) | **GET** /tag/{id} | Get tags for a resource ID
[**tag_id_patch**](TagApi.md#tag_id_patch) | **PATCH** /tag/{id} | Removes tags for a resource ID
[**tag_id_put**](TagApi.md#tag_id_put) | **PUT** /tag/{id} | Set tags for a resource ID



## tag_id_get

> ::std::collections::HashMap<String, String> tag_id_get(id)
Get tags for a resource ID

Returns tag information for a resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of resource | [required] |

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tag_id_patch

> ::std::collections::HashMap<String, String> tag_id_patch(id, tag)
Removes tags for a resource ID

Removes tags from a resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of resource | [required] |
**tag** | [**Tag**](Tag.md) |  | [required] |

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tag_id_put

> ::std::collections::HashMap<String, String> tag_id_put(id, tag)
Set tags for a resource ID

Adds tags to a resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of resource | [required] |
**tag** | [**Tag**](Tag.md) |  | [required] |

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

