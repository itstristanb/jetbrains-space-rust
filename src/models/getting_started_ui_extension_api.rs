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
pub struct GettingStartedUiExtensionApi {
    #[serde(rename = "gettingStartedUrl")]
    pub getting_started_url: String,
    #[serde(rename = "gettingStartedTitle")]
    pub getting_started_title: String,
    #[serde(rename = "openInNewTab", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub open_in_new_tab: Option<Option<bool>>,
}

impl GettingStartedUiExtensionApi {
    pub fn new(getting_started_url: String, getting_started_title: String) -> GettingStartedUiExtensionApi {
        GettingStartedUiExtensionApi {
            getting_started_url,
            getting_started_title,
            open_in_new_tab: None,
        }
    }
}


