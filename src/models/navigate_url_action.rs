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
pub struct NavigateUrlAction {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "withBackUrl")]
    pub with_back_url: bool,
    #[serde(rename = "openInNewTab")]
    pub open_in_new_tab: bool,
}

impl NavigateUrlAction {
    pub fn new(url: String, with_back_url: bool, open_in_new_tab: bool) -> NavigateUrlAction {
        NavigateUrlAction {
            url,
            with_back_url,
            open_in_new_tab,
        }
    }
}

