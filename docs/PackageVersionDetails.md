# PackageVersionDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**class_name** | **String** |  | 
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
**description** | Option<**String**> |  | 
**homepage** | Option<**String**> |  | [optional]
**dependencies** | [**Vec<crate::models::NpmPackageDependency>**](NpmPackageDependency.md) |  | 
**keywords** | **Vec<String>** |  | 
**license** | Option<**String**> |  | 
**project_url** | Option<**String**> |  | [optional]
**repository_url** | Option<**String**> |  | 
**repository_revision** | Option<**String**> |  | [optional]
**readme** | Option<**String**> |  | 
**documentation** | **String** |  | 
**categories** | **Vec<String>** |  | 
**license_file_content** | Option<**String**> |  | [optional]
**git_repository** | Option<**String**> |  | [optional]
**links** | Option<**String**> |  | [optional]
**home_page** | Option<**String**> |  | 
**issue_tracker** | **String** |  | 
**changelog** | **String** |  | 
**dev_dependencies** | Option<[**Vec<crate::models::DartPackageDependency>**](DartPackageDependency.md)> |  | [optional]
**dependency_overrides** | Option<[**Vec<crate::models::DartPackageDependency>**](DartPackageDependency.md)> |  | [optional]
**environment** | Option<[**Vec<crate::models::DartPackageDependency>**](DartPackageDependency.md)> |  | [optional]
**not_normalized_name** | **String** |  | 
**platform** | Option<**String**> |  | [optional]
**summary** | Option<**String**> |  | [optional]
**description_content_type** | Option<**String**> |  | [optional]
**author_from_package_details** | Option<**String**> |  | [optional]
**author_email** | Option<**String**> |  | [optional]
**maintainer** | Option<**String**> |  | [optional]
**maintainer_email** | Option<**String**> |  | [optional]
**classifiers** | **Vec<String>** |  | 
**project_urls** | [**Vec<crate::models::PythonPackageUrl>**](PythonPackageUrl.md) |  | 
**requires_dist** | **Vec<String>** |  | 
**requires_python** | Option<**String**> |  | [optional]
**files** | [**Vec<crate::models::MavenPackageFile>**](MavenPackageFile.md) |  | 
**verbatim_version** | **String** |  | 
**license_url** | Option<**String**> |  | [optional]
**icon** | Option<**String**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**packaging** | Option<**String**> |  | [optional]
**package_name** | Option<**String**> |  | [optional]
**url** | Option<**String**> |  | [optional]
**licenses** | **Vec<String>** |  | 
**scm_url** | Option<**String**> |  | [optional]
**kotlin_platforms** | Option<[**Vec<crate::models::KotlinPlatform>**](KotlinPlatform.md)> |  | [optional]
**parent** | Option<[**crate::models::MavenPackageParent**](MavenPackageParent.md)> |  | [optional]
**path_prefix** | Option<**String**> |  | [optional]
**unity_version** | Option<**String**> |  | [optional]
**schema_version** | **i32** |  | 
**media_type** | **String** |  | 
**manifest_type** | **String** |  | 
**image** | Option<[**crate::models::ContainerImage**](ContainerImage.md)> |  | [optional]
**chart** | Option<[**crate::models::ContainerHelmChart**](ContainerHelmChart.md)> |  | [optional]
**subject** | Option<[**crate::models::PackageVersionRef**](PackageVersionRef.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


