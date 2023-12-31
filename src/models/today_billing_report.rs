/*
 * Space
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023.3.0-DEV.171131
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TodayBillingReport {
    #[serde(rename = "plan")]
    pub plan: Box<crate::models::PurchasedBillingPlan>,
    #[serde(rename = "activeUsers")]
    pub active_users: i32,
    #[serde(rename = "userUsage")]
    pub user_usage: i32,
    #[serde(rename = "userCost")]
    pub user_cost: f64,
    #[serde(rename = "activeGuests", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub active_guests: Option<Option<i32>>,
    #[serde(rename = "paidActiveGuests", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub paid_active_guests: Option<Option<i32>>,
    #[serde(rename = "freeActiveGuests", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub free_active_guests: Option<Option<i32>>,
    #[serde(rename = "guestCost", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub guest_cost: Option<Option<f64>>,
    #[serde(rename = "activeMembers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub active_members: Option<Option<i32>>,
    #[serde(rename = "activeLegacyExternalCollaborators", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub active_legacy_external_collaborators: Option<Option<i32>>,
    #[serde(rename = "activeExternalCollaborators", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub active_external_collaborators: Option<Option<i32>>,
    #[serde(rename = "storageAllocationB", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub storage_allocation_b: Option<Option<i64>>,
    #[serde(rename = "storageTotalUsage")]
    pub storage_total_usage: i64,
    #[serde(rename = "storageCost")]
    pub storage_cost: f64,
    #[serde(rename = "bandwidthTotalUsage")]
    pub bandwidth_total_usage: i64,
    #[serde(rename = "bandwidthCost")]
    pub bandwidth_cost: f64,
    #[serde(rename = "ciUsage")]
    pub ci_usage: i64,
    #[serde(rename = "ciCost")]
    pub ci_cost: f64,
    #[serde(rename = "appUsage")]
    pub app_usage: i64,
    #[serde(rename = "chatUsage")]
    pub chat_usage: i64,
    #[serde(rename = "totalCost")]
    pub total_cost: f64,
}

impl TodayBillingReport {
    pub fn new(plan: crate::models::PurchasedBillingPlan, active_users: i32, user_usage: i32, user_cost: f64, storage_total_usage: i64, storage_cost: f64, bandwidth_total_usage: i64, bandwidth_cost: f64, ci_usage: i64, ci_cost: f64, app_usage: i64, chat_usage: i64, total_cost: f64) -> TodayBillingReport {
        TodayBillingReport {
            plan: Box::new(plan),
            active_users,
            user_usage,
            user_cost,
            active_guests: None,
            paid_active_guests: None,
            free_active_guests: None,
            guest_cost: None,
            active_members: None,
            active_legacy_external_collaborators: None,
            active_external_collaborators: None,
            storage_allocation_b: None,
            storage_total_usage,
            storage_cost,
            bandwidth_total_usage,
            bandwidth_cost,
            ci_usage,
            ci_cost,
            app_usage,
            chat_usage,
            total_cost,
        }
    }
}


