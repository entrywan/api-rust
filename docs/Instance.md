# Instance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**state** | Option<**String**> | Instance state | [optional]
**hostname** | Option<**String**> | Instance hostname | [optional]
**account** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Instance account owner | [optional]
**ip4** | Option<**String**> |  | [optional]
**location** | Option<**String**> | Instance datacenter location | [optional]
**cpus** | Option<[**crate::models::Int**](int.md)> | Instance CPU cores | [optional]
**ram** | Option<[**crate::models::Int**](int.md)> | Instance memory in gigabytes (GB) | [optional]
**disk** | Option<[**crate::models::Int**](int.md)> | Instance hard drive size in gigabytes (GB) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


