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
pub struct MarketplaceAppMetadata {
    #[serde(rename = "marketplaceAppId")]
    pub marketplace_app_id: String,
    #[serde(rename = "lastSentServerUrl")]
    pub last_sent_server_url: String,
    #[serde(rename = "connectionStatus")]
    pub connection_status: crate::models::AppConnectionStatus,
    #[serde(rename = "uninstallationStatus", skip_serializing_if = "Option::is_none")]
    pub uninstallation_status: Option<crate::models::AppUninstallationStatus>,
    #[serde(rename = "uninstallationStartedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uninstallation_started_at: Option<Option<String>>,
}

impl MarketplaceAppMetadata {
    pub fn new(marketplace_app_id: String, last_sent_server_url: String, connection_status: crate::models::AppConnectionStatus) -> MarketplaceAppMetadata {
        MarketplaceAppMetadata {
            marketplace_app_id,
            last_sent_server_url,
            connection_status,
            uninstallation_status: None,
            uninstallation_started_at: None,
        }
    }
}


