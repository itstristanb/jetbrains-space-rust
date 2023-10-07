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
pub struct ApplicationHomepageUiExtensionIn {
    #[serde(rename = "iframeUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub iframe_url: Option<Option<String>>,
}

impl ApplicationHomepageUiExtensionIn {
    pub fn new() -> ApplicationHomepageUiExtensionIn {
        ApplicationHomepageUiExtensionIn {
            iframe_url: None,
        }
    }
}

