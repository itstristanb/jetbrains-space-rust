# PythonPackageVersionDetails

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
**not_normalized_name** | **String** |  | 
**platform** | Option<**String**> |  | [optional]
**summary** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**description_content_type** | Option<**String**> |  | [optional]
**keywords** | **Vec<String>** |  | 
**home_page** | Option<**String**> |  | [optional]
**author_from_package_details** | Option<**String**> |  | [optional]
**author_email** | Option<**String**> |  | [optional]
**maintainer** | Option<**String**> |  | [optional]
**maintainer_email** | Option<**String**> |  | [optional]
**license** | Option<**String**> |  | [optional]
**classifiers** | **Vec<String>** |  | 
**project_urls** | [**Vec<crate::models::PythonPackageUrl>**](PythonPackageUrl.md) |  | 
**requires_dist** | **Vec<String>** |  | 
**requires_python** | Option<**String**> |  | [optional]
**files** | [**Vec<crate::models::PythonPackageFile>**](PythonPackageFile.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


