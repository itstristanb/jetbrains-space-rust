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
pub struct McDropDownButton {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<crate::models::MessageButtonStyle>,
    #[serde(rename = "dropdownItems")]
    pub dropdown_items: Vec<crate::models::McButtonDropDownItem>,
    #[serde(rename = "disabled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<Option<bool>>,
}

impl McDropDownButton {
    pub fn new(text: String, dropdown_items: Vec<crate::models::McButtonDropDownItem>) -> McDropDownButton {
        McDropDownButton {
            text,
            style: None,
            dropdown_items,
            disabled: None,
        }
    }
}


