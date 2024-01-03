# \SsHkeyApi

All URIs are relative to *https://api.entrywan.com/v1beta*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sshkey_get**](SsHkeyApi.md#sshkey_get) | **GET** /sshkey | List SSH keys
[**sshkey_id_delete**](SsHkeyApi.md#sshkey_id_delete) | **DELETE** /sshkey/{id} | Delete SSH key
[**sshkey_post**](SsHkeyApi.md#sshkey_post) | **POST** /sshkey | Create SSH key



## sshkey_get

> Vec<crate::models::SsHkey> sshkey_get()
List SSH keys

List SSH keys

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::SsHkey>**](SSHkey.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sshkey_id_delete

> sshkey_id_delete(id)
Delete SSH key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of SSH key | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sshkey_post

> sshkey_post(sshkey_post_request)
Create SSH key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sshkey_post_request** | [**SshkeyPostRequest**](SshkeyPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

