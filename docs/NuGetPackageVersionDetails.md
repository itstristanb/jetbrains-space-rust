# NuGetPackageVersionDetails

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
**verbatim_version** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**project_url** | Option<**String**> |  | [optional]
**license** | Option<**String**> |  | [optional]
**license_url** | Option<**String**> |  | [optional]
**icon** | Option<**String**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**dependencies** | [**Vec<crate::models::NuGetDependency>**](NuGetDependency.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


