# LoadBalancer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**account** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Load balancer account owner | [optional]
**created** | Option<**String**> | Load balancer creation date | [optional]
**name** | Option<**String**> |  | [optional]
**location** | Option<**String**> |  | [optional]
**ip** | Option<**String**> |  | [optional]
**listeners** | Option<[**Vec<crate::models::LoadbalancerPostRequestListenersInner>**](_loadbalancer_post_request_listeners_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


