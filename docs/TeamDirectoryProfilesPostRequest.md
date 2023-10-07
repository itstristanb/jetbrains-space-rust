# TeamDirectoryProfilesPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | **String** |  | 
**first_name** | **String** |  | 
**last_name** | **String** |  | 
**emails** | Option<**Vec<String>**> |  | [optional][default to []]
**phones** | Option<**Vec<String>**> |  | [optional][default to []]
**birthday** | Option<**String**> |  | [optional]
**about** | Option<**String**> |  | [optional]
**messengers** | Option<**Vec<String>**> |  | [optional][default to []]
**links** | Option<**Vec<String>**> |  | [optional][default to []]
**not_a_member** | Option<**bool**> |  | [optional][default to false]
**joined** | Option<**String**> |  | [optional]
**left** | Option<**String**> |  | [optional]
**left_at** | Option<**String**> |  | [optional]
**speaks_english** | Option<**bool**> |  | [optional]
**picture_attachment_id** | Option<**String**> |  | [optional]
**avatar_crop_square** | Option<[**crate::models::AvatarCropSquare**](AvatarCropSquare.md)> |  | [optional]
**custom_field_values** | Option<[**Vec<crate::models::CustomFieldInputValue>**](CustomFieldInputValue.md)> |  | [optional][default to []]
**external_id** | Option<**String**> |  | [optional]
**location** | Option<**String**> |  | [optional]
**guest** | Option<**bool**> |  | [optional]
**guest_type** | Option<[**crate::models::GuestType**](GuestType.md)> |  | [optional]
**project** | Option<[**crate::models::ProjectIdentifier**](ProjectIdentifier.md)> |  | [optional]
**project_role** | Option<[**crate::models::ProjectTeamRole**](ProjectTeamRole.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


