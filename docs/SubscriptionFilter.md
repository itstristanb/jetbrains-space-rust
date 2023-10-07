# SubscriptionFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**class_name** | **String** |  | 
**project** | Option<[**crate::models::PrProject**](PR_Project.md)> |  | [optional]
**repository** | Option<**String**> |  | 
**spec** | [**crate::models::RepoCommitsSubscriptionFilterSpec**](RepoCommitsSubscriptionFilterSpec.md) |  | 
**reasons** | [**Vec<crate::models::AbsenceReasonRecord>**](AbsenceReasonRecord.md) |  | 
**folders** | Option<[**Vec<crate::models::DocumentFolder>**](DocumentFolder.md)> |  | [optional]
**organizers** | Option<[**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md)> |  | [optional]
**participants** | [**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md) |  | 
**teams** | [**Vec<crate::models::TdTeam>**](TD_Team.md) |  | 
**locations** | [**Vec<crate::models::TdLocation>**](TD_Location.md) |  | 
**documents** | [**Vec<crate::models::Document>**](Document.md) |  | 
**name_pattern** | **Vec<String>** |  | 
**version_pattern** | Option<**String**> |  | [optional]
**profiles** | Option<[**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md)> |  | [optional]
**projects** | [**Vec<crate::models::PrProject>**](PR_Project.md) |  | 
**repository_name** | Option<**String**> |  | [optional]
**branch_spec** | Option<**Vec<String>**> |  | 
**jobs** | Option<**Vec<String>**> |  | [optional]
**authors** | [**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md) |  | 
**path_spec** | **Vec<String>** |  | 
**title_regex** | **String** |  | 
**emojis** | **Vec<String>** |  | 
**repositories** | Option<**Vec<String>**> |  | [optional]
**target_identifiers** | Option<**Vec<String>**> |  | [optional]
**application** | Option<[**crate::models::EsApp**](ES_App.md)> |  | [optional]
**channel** | Option<**String**> |  | [optional]
**contact** | Option<[**crate::models::ChatContactRecord**](ChatContactRecord.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


