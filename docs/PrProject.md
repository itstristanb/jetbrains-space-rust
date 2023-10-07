# PrProject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**key** | [**crate::models::ProjectKey**](ProjectKey.md) |  | 
**name** | **String** |  | 
**private** | **bool** |  | 
**description** | Option<**String**> |  | [optional]
**icon** | Option<**String**> |  | [optional]
**latest_repository_activity** | Option<**String**> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**archived** | **bool** |  | 
**guest_profiles** | [**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md) |  | 
**r#type** | [**crate::models::ProjectTeamType**](ProjectTeamType.md) |  | 
**teams** | Option<[**Vec<crate::models::TdTeam>**](TD_Team.md)> |  | [optional]
**members** | Option<[**Vec<crate::models::ProjectTeamMemberRecord>**](ProjectTeamMemberRecord.md)> |  | [optional]
**team** | Option<[**crate::models::TdTeam**](TD_Team.md)> |  | [optional]
**features** | Option<[**Vec<crate::models::ProjectFeatureState>**](ProjectFeatureState.md)> |  | [optional]
**features_usage** | Option<[**Vec<crate::models::ProjectFeatureUsage>**](ProjectFeatureUsage.md)> |  | [optional]
**boards** | [**Vec<crate::models::BoardRecord>**](BoardRecord.md) |  | 
**repos** | [**Vec<crate::models::PrRepositoryInfo>**](PR_RepositoryInfo.md) |  | 
**personal_feature_pins** | Option<[**Vec<crate::models::ToggleableProjectFeaturePins>**](ToggleableProjectFeaturePins.md)> |  | [optional]
**enable** | **bool** |  | 
**hours_in_day** | **i32** |  | 
**days_in_week** | **i32** |  | 
**format** | [**crate::models::DurationTextFormat**](DurationTextFormat.md) |  | 
**tags** | **Vec<String>** |  | 
**packages** | [**Vec<crate::models::ProjectPackageRepository>**](ProjectPackageRepository.md) |  | 
**collaborators_profiles** | [**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md) |  | 
**collaborators_teams** | [**Vec<crate::models::TdTeam>**](TD_Team.md) |  | 
**trackers** | [**Vec<crate::models::ProjectIssueTrackerItem>**](ProjectIssueTrackerItem.md) |  | 
**bundles** | [**Vec<crate::models::ProjectParameterBundle>**](ProjectParameterBundle.md) |  | 
**feature_pins** | Option<[**Vec<crate::models::CommonProjectFeaturePins>**](CommonProjectFeaturePins.md)> |  | [optional]
**member_profiles** | [**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md) |  | 
**member_teams** | [**Vec<crate::models::TdTeam>**](TD_Team.md) |  | 
**admin_profiles** | [**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md) |  | 
**admin_teams** | [**Vec<crate::models::TdTeam>**](TD_Team.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


