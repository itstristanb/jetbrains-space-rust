# ApplicationsApplicationWebhooksPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**endpoint** | Option<[**crate::models::EndpointCreateDto**](EndpointCreateDTO.md)> |  | [optional]
**endpoint_auth** | Option<[**crate::models::EndpointAuthCreateDto**](EndpointAuthCreateDTO.md)> |  | [optional]
**enabled** | Option<**bool**> |  | [optional][default to true]
**accepted_http_response_codes** | Option<**Vec<i32>**> |  | [optional][default to []]
**do_retries** | Option<**bool**> |  | [optional][default to true]
**payload_fields** | Option<**String**> |  | [optional]
**payload_template** | Option<**String**> |  | [optional]
**subscriptions** | Option<[**Vec<crate::models::SubscriptionDefinition>**](SubscriptionDefinition.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


