# WebhookRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**archived** | **bool** |  | 
**app** | [**crate::models::EsApp**](ES_App.md) |  | 
**subscriptions** | [**Vec<crate::models::SubscriptionDto>**](SubscriptionDTO.md) |  | 
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**use_app_endpoint** | **bool** |  | 
**endpoint** | [**crate::models::EndpointDto**](EndpointDTO.md) |  | 
**use_app_endpoint_auth** | **bool** |  | 
**endpoint_auth** | [**crate::models::EndpointAuthDto**](EndpointAuthDTO.md) |  | 
**enabled** | **bool** |  | 
**accepted_http_response_codes** | **Vec<i32>** |  | 
**do_retries** | **bool** |  | 
**payload_fields** | Option<**String**> |  | [optional]
**payload_template** | Option<**String**> |  | [optional]
**payload_type** | Option<[**crate::models::PayloadType**](PayloadType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


