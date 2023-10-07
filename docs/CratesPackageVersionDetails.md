# CratesPackageVersionDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
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
**dependencies** | [**Vec<crate::models::CratesPackageDependency>**](CratesPackageDependency.md) |  | 
**description** | Option<**String**> |  | [optional]
**documentation** | Option<**String**> |  | [optional]
**homepage** | Option<**String**> |  | [optional]
**readme** | Option<**String**> |  | [optional]
**keywords** | **Vec<String>** |  | 
**categories** | **Vec<String>** |  | 
**license** | Option<**String**> |  | [optional]
**license_file_content** | Option<**String**> |  | [optional]
**git_repository** | Option<**String**> |  | [optional]
**links** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


