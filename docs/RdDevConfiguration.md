# RdDevConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** |  | 
**repo_connections** | [**Vec<crate::models::RepoConnectionWithBranch>**](RepoConnectionWithBranch.md) |  | 
**dev_container** | [**crate::models::RdDevContainer**](RdDevContainer.md) |  | 
**ide** | [**crate::models::RdDevConfigurationIde**](RdDevConfigurationIde.md) |  | 
**instance_type_name** | **String** |  | 
**project** | [**crate::models::PrProject**](PR_Project.md) |  | 
**hot_pool** | Option<[**crate::models::DevConfigurationHotPool**](DevConfigurationHotPool.md)> |  | [optional]
**warmup** | [**crate::models::DevConfigurationWarmup**](DevConfigurationWarmup.md) |  | 
**hibernation** | Option<[**crate::models::DevConfigurationHibernation**](DevConfigurationHibernation.md)> |  | [optional]
**project_root** | Option<**String**> |  | [optional]
**ssh_enabled** | Option<**bool**> |  | [optional]
**access** | Option<[**crate::models::DevConfigurationAccessSettingsDto**](DevConfigurationAccessSettingsDTO.md)> |  | [optional]
**archived** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


