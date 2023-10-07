# ContainerPackageVersionDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**crate::models::PackageType**](PackageType.md) |  | 
**repository** | **String** |  | 
**name** | **String** |  | 
**version** | **String** |  | 
**tags** | Option<**Vec<String>**> |  | [optional]
**created** | **i64** |  | 
**accessed** | Option<**i64**> |  | [optional]
**downloads** | **i64** |  | 
**pinned** | **bool** |  | 
**comment** | Option<**String**> |  | [optional]
**disk_size** | **i64** |  | 
**author** | Option<[**crate::models::CPrincipal**](CPrincipal.md)> |  | [optional]
**authors** | Option<[**Vec<crate::models::CPrincipal>**](CPrincipal.md)> |  | [optional]
**origin** | Option<[**crate::models::PackageOrigin**](PackageOrigin.md)> |  | [optional]
**metadata** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**schema_version** | **i32** |  | 
**media_type** | **String** |  | 
**manifest_type** | **String** |  | 
**image** | Option<[**crate::models::ContainerImage**](ContainerImage.md)> |  | [optional]
**chart** | Option<[**crate::models::ContainerHelmChart**](ContainerHelmChart.md)> |  | [optional]
**subject** | Option<[**crate::models::PackageVersionRef**](PackageVersionRef.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


