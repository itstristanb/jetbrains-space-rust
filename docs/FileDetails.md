# FileDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**crate::models::PackageType**](PackageType.md) |  | 
**repository** | **String** |  | 
**name** | **String** |  | 
**created** | **i64** |  | 
**created_by** | Option<[**crate::models::CPrincipal**](CPrincipal.md)> |  | [optional]
**last_modified** | Option<**i64**> |  | [optional]
**downloads** | **i64** |  | 
**disk_size** | **i64** |  | 
**authors** | Option<[**Vec<crate::models::CPrincipal>**](CPrincipal.md)> |  | [optional]
**origin** | Option<[**crate::models::PackageOrigin**](PackageOrigin.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**path** | **String** |  | 
**content_type** | Option<**String**> |  | [optional]
**metadata** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


