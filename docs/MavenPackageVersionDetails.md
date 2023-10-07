# MavenPackageVersionDetails

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
**packaging** | Option<**String**> |  | [optional]
**package_name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**url** | Option<**String**> |  | [optional]
**licenses** | **Vec<String>** |  | 
**scm_url** | Option<**String**> |  | [optional]
**dependencies** | [**Vec<crate::models::MavenPackageDependency>**](MavenPackageDependency.md) |  | 
**kotlin_platforms** | Option<[**Vec<crate::models::KotlinPlatform>**](KotlinPlatform.md)> |  | [optional]
**parent** | Option<[**crate::models::MavenPackageParent**](MavenPackageParent.md)> |  | [optional]
**path_prefix** | Option<**String**> |  | [optional]
**files** | [**Vec<crate::models::MavenPackageFile>**](MavenPackageFile.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


