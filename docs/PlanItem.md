# PlanItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**checklist_id** | **String** |  | 
**tag** | Option<[**crate::models::PlanningTag**](PlanningTag.md)> |  | [optional]
**simple_text** | Option<**String**> |  | [optional]
**simple_done** | Option<**bool**> |  | [optional]
**issue** | Option<[**crate::models::Issue**](Issue.md)> |  | [optional]
**issue_problem** | Option<**String**> |  | [optional]
**can_edit_issue** | Option<**bool**> |  | [optional]
**has_children** | **bool** |  | 
**archived** | **bool** |  | 
**topics** | [**Vec<crate::models::Topic>**](Topic.md) |  | 
**children** | [**Vec<crate::models::PlanItem>**](PlanItem.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


