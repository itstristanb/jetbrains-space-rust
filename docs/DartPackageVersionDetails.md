# DartPackageVersionDetails

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
**description** | **String** |  | 
**home_page** | **String** |  | 
**repository_url** | **String** |  | 
**issue_tracker** | **String** |  | 
**documentation** | **String** |  | 
**license** | **String** |  | 
**readme** | **String** |  | 
**changelog** | **String** |  | 
**dependencies** | Option<[**Vec<crate::models::DartPackageDependency>**](DartPackageDependency.md)> |  | [optional]
**dev_dependencies** | Option<[**Vec<crate::models::DartPackageDependency>**](DartPackageDependency.md)> |  | [optional]
**dependency_overrides** | Option<[**Vec<crate::models::DartPackageDependency>**](DartPackageDependency.md)> |  | [optional]
**environment** | Option<[**Vec<crate::models::DartPackageDependency>**](DartPackageDependency.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


