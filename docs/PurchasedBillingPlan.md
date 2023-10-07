# PurchasedBillingPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**jet_sales_id** | Option<**String**> |  | [optional]
**license_issuer** | Option<**String**> |  | [optional]
**installation_public_key** | Option<**String**> |  | [optional]
**plan** | **String** |  | 
**billing_period** | **String** |  | 
**since** | **String** |  | 
**till** | **String** |  | 
**may_use_till** | Option<**String**> |  | [optional]
**currency** | [**crate::models::Currency**](Currency.md) |  | 
**add_user_price** | **f64** |  | 
**add_storage_price** | **f64** |  | 
**add_bandwidth_price** | **f64** |  | 
**add_ci_credit_price** | **f64** |  | 
**min_active_users** | **i32** |  | 
**prepaid_users** | **i32** |  | 
**storage_per_user** | **i32** |  | 
**storage_overall** | **i32** |  | 
**bandwidth_per_user** | **i32** |  | 
**bandwidth_overall** | **i32** |  | 
**ci_credits** | **i32** |  | 
**ci_credits_reserve** | **i32** |  | 
**ci_credits_rate_for_external_worker** | Option<**f64**> |  | [optional]
**integrations** | **i32** |  | 
**search_history** | **i32** |  | 
**balance** | **f64** |  | 
**hard_limit_amount** | **f64** |  | 
**has_card_verified_admin** | Option<**bool**> |  | [optional]
**is_trial** | Option<**bool**> |  | [optional]
**spent_trials** | Option<**Vec<String>**> |  | [optional]
**trial_base_plan** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


