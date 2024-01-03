# InstancePostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu** | **i32** | Number of CPU cores | 
**ram** | **i32** | Gigabytes (GB) of memory | 
**disk** | **i32** | Hard drive size in gigabytes (GB) | 
**location** | **String** | Data center location | 
**userdata** | Option<**String**> | Initial script to be performed on first boot | [optional]
**sshkey** | **String** | Name of ssh key to be planted on instance for root user | 
**os** | Option<**String**> | Name of operating system image | [optional]
**hostname** | Option<**String**> | Hostname | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


