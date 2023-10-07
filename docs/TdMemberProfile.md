# TdMemberProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**username** | **String** |  | 
**name** | [**crate::models::TdProfileName**](TD_ProfileName.md) |  | 
**speaks_english** | **bool** |  | 
**small_avatar** | Option<**String**> |  | [optional]
**avatar** | Option<**String**> |  | [optional]
**profile_picture** | Option<**String**> |  | [optional]
**languages** | [**Vec<crate::models::TdProfileLanguage>**](TD_ProfileLanguage.md) |  | 
**archived** | **bool** |  | 
**not_a_member** | **bool** |  | 
**suspended** | Option<**bool**> |  | [optional]
**suspended_at** | Option<**String**> |  | [optional]
**joined** | Option<**String**> |  | [optional]
**left_at** | Option<**String**> |  | [optional]
**external** | Option<**bool**> |  | [optional]
**external_light** | Option<**bool**> |  | [optional]
**managers** | [**Vec<crate::models::TdMemberProfile>**](TD_MemberProfile.md) |  | 
**onboarding_required** | **bool** |  | 
**show_banner_on_landing_page** | Option<**bool**> |  | [optional]
**show_banner_on_project_page** | Option<**bool**> |  | [optional]
**show_banner_on_team_directory_home_page** | Option<**bool**> |  | [optional]
**access_suspended** | Option<**bool**> |  | [optional]
**access_suspended_at** | Option<**String**> |  | [optional]
**left** | Option<**String**> |  | [optional]
**about** | Option<**String**> |  | [optional]
**avatar_crop_square** | Option<[**crate::models::AvatarCropSquare**](AvatarCropSquare.md)> |  | [optional]
**gender** | Option<[**crate::models::Gender**](Gender.md)> |  | [optional]
**birthday** | Option<**String**> |  | [optional]
**memberships** | [**Vec<crate::models::TdMembership>**](TD_Membership.md) |  | 
**unapproved_memberships** | Option<[**Vec<crate::models::TdMembership>**](TD_Membership.md)> |  | [optional]
**logins** | [**Vec<crate::models::EsProfileLogin>**](ES_ProfileLogin.md) |  | 
**membership_history** | [**Vec<crate::models::TdMembership>**](TD_Membership.md) |  | 
**status** | [**crate::models::TwoFactorAuthenticationStatus**](TwoFactorAuthenticationStatus.md) |  | 
**external_id** | Option<**String**> |  | [optional]
**locations** | [**Vec<crate::models::TdMemberLocation>**](TD_MemberLocation.md) |  | 
**absences** | [**Vec<crate::models::AbsenceRecord>**](AbsenceRecord.md) |  | 
**topics** | [**Vec<crate::models::Topic>**](Topic.md) |  | 
**emails** | [**Vec<crate::models::TdProfileEmail>**](TD_ProfileEmail.md) |  | 
**phones** | **Vec<String>** |  | 
**messengers** | **Vec<String>** |  | 
**links** | **Vec<String>** |  | 
**folder_with_children** | [**crate::models::DocumentFolderWithChildren**](DocumentFolderWithChildren.md) |  | 
**location_history** | [**Vec<crate::models::TdMemberLocation>**](TD_MemberLocation.md) |  | 
**custom_fields** | [**::std::collections::HashMap<String, crate::models::CfValue>**](CFValue.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


