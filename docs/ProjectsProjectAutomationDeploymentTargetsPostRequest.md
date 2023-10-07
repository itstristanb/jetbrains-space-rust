# ProjectsProjectAutomationDeploymentTargetsPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** |  | 
**name** | **String** |  | 
**description** | **String** |  | 
**repositories** | Option<[**Vec<crate::models::DeployTargetRepositoryDto>**](DeployTargetRepositoryDTO.md)> |  | [optional][default to []]
**manual_control** | Option<**bool**> |  | [optional]
**single_scheduled** | Option<**bool**> |  | [optional]
**hang_timeout_minutes** | Option<**i32**> |  | [optional]
**fail_timeout_minutes** | Option<**i32**> |  | [optional]
**responsible_users** | Option<**Vec<String>**> |  | [optional]
**responsible_teams** | Option<**Vec<String>**> |  | [optional]
**links** | Option<[**Vec<crate::models::DeployTargetLink>**](DeployTargetLink.md)> |  | [optional]
**custom_fields** | Option<[**Vec<crate::models::CustomFieldInputValue>**](CustomFieldInputValue.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


