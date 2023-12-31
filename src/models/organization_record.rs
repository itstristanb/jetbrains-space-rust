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
pub struct OrganizationRecord {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "orgId")]
    pub org_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slogan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub slogan: Option<Option<String>>,
    #[serde(rename = "logoId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub logo_id: Option<Option<String>>,
    #[serde(rename = "onboardingRequired", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub onboarding_required: Option<Option<bool>>,
    #[serde(rename = "allowDomainsEdit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub allow_domains_edit: Option<Option<bool>>,
    #[serde(rename = "createdAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Option<i64>>,
    #[serde(rename = "createdWithNavigationV2", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created_with_navigation_v2: Option<Option<bool>>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<Box<crate::models::ATimeZone>>,
    #[serde(rename = "orgSize", skip_serializing_if = "Option::is_none")]
    pub org_size: Option<crate::models::OrgSizeDto>,
    #[serde(rename = "orgIndustry", skip_serializing_if = "Option::is_none")]
    pub org_industry: Option<crate::models::OrgIndustryDto>,
    #[serde(rename = "sendAnonymousDataAgreementAccepted", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub send_anonymous_data_agreement_accepted: Option<Option<bool>>,
    #[serde(rename = "marketplaceEnabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub marketplace_enabled: Option<Option<bool>>,
    #[serde(rename = "slackWorkspace", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub slack_workspace: Option<Option<String>>,
}

impl OrganizationRecord {
    pub fn new(id: String, org_id: String, name: String) -> OrganizationRecord {
        OrganizationRecord {
            id,
            org_id,
            name,
            slogan: None,
            logo_id: None,
            onboarding_required: None,
            allow_domains_edit: None,
            created_at: None,
            created_with_navigation_v2: None,
            timezone: None,
            org_size: None,
            org_industry: None,
            send_anonymous_data_agreement_accepted: None,
            marketplace_enabled: None,
            slack_workspace: None,
        }
    }
}


