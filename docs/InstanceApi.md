# \InstanceApi

All URIs are relative to *https://api.entrywan.com/v1beta*

Method | HTTP request | Description
------------- | ------------- | -------------
[**instance_get**](InstanceApi.md#instance_get) | **GET** /instance | List instances
[**instance_id_delete**](InstanceApi.md#instance_id_delete) | **DELETE** /instance/{id} | Delete an instance
[**instance_id_get**](InstanceApi.md#instance_id_get) | **GET** /instance/{id} | Get instance by ID
[**instance_id_reboot_patch**](InstanceApi.md#instance_id_reboot_patch) | **PATCH** /instance/{id}/reboot | Reboot an instance
[**instance_id_snapshot_get**](InstanceApi.md#instance_id_snapshot_get) | **GET** /instance/{id}/snapshot | List instance snapshots
[**instance_id_snapshot_name_delete**](InstanceApi.md#instance_id_snapshot_name_delete) | **DELETE** /instance/{id}/snapshot/{name} | Delete a snapshot
[**instance_id_snapshot_name_put**](InstanceApi.md#instance_id_snapshot_name_put) | **PUT** /instance/{id}/snapshot/{name} | Revert to snapshot
[**instance_id_snapshot_put**](InstanceApi.md#instance_id_snapshot_put) | **PUT** /instance/{id}/snapshot | Snapshot an instance
[**instance_id_start_patch**](InstanceApi.md#instance_id_start_patch) | **PATCH** /instance/{id}/start | Start an instance
[**instance_id_stop_patch**](InstanceApi.md#instance_id_stop_patch) | **PATCH** /instance/{id}/stop | Stop an instance
[**instance_post**](InstanceApi.md#instance_post) | **POST** /instance | Add a new instance



## instance_get

> Vec<crate::models::Instance> instance_get()
List instances

Return a list of all compute instances for an account.  Includes instances in any state, including stopped instances.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Instance>**](Instance.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_id_delete

> instance_id_delete(id)
Delete an instance



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Instance id | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_id_get

> crate::models::Instance instance_id_get(id)
Get instance by ID

Returns a single instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Instance ID | [required] |

### Return type

[**crate::models::Instance**](Instance.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_id_reboot_patch

> instance_id_reboot_patch(id)
Reboot an instance

Reboot an instance

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


## instance_id_snapshot_get

> Vec<String> instance_id_snapshot_get(id)
List instance snapshots

List instance snapshots.  The name of each snapshot is the UNIX timestamp when it was taken.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Instance ID | [required] |

### Return type

**Vec<String>**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_id_snapshot_name_delete

> instance_id_snapshot_name_delete(id, name)
Delete a snapshot

Delete a snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Instance ID | [required] |
**name** | **String** | Name of snapshot | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_id_snapshot_name_put

> instance_id_snapshot_name_put(id, name)
Revert to snapshot

Revert an instance to its state to that of a snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Instance ID | [required] |
**name** | **String** | Name of snapshot | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_id_snapshot_put

> instance_id_snapshot_put(id)
Snapshot an instance

Snapshot an instance

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


## instance_id_start_patch

> instance_id_start_patch(id)
Start an instance

Start an instance

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


## instance_id_stop_patch

> instance_id_stop_patch(id)
Stop an instance

Stop an instance

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


## instance_post

> crate::models::InstancePost200Response instance_post(instance_post_request)
Add a new instance

Creates an instance and performs the initial boot.  After about 5 seconds, the instance is in _running_ state and ready to accept incoming SSH connections on port 22.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_post_request** | [**InstancePostRequest**](InstancePostRequest.md) |  | [required] |

### Return type

[**crate::models::InstancePost200Response**](_instance_post_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

