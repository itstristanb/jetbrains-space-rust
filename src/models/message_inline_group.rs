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
pub struct MessageInlineGroup {
    #[serde(rename = "accessory", skip_serializing_if = "Option::is_none")]
    pub accessory: Option<Box<crate::models::MessageAccessoryElement>>,
    #[serde(rename = "elements")]
    pub elements: Vec<crate::models::MessageInlineElement>,
    #[serde(rename = "textSize", skip_serializing_if = "Option::is_none")]
    pub text_size: Option<crate::models::MessageTextSize>,
    #[serde(rename = "textStyle", skip_serializing_if = "Option::is_none")]
    pub text_style: Option<crate::models::MessageStyle>,
    #[serde(rename = "elementsStyle", skip_serializing_if = "Option::is_none")]
    pub elements_style: Option<crate::models::MessageStyle>,
}

impl MessageInlineGroup {
    pub fn new(elements: Vec<crate::models::MessageInlineElement>) -> MessageInlineGroup {
        MessageInlineGroup {
            accessory: None,
            elements,
            text_size: None,
            text_style: None,
            elements_style: None,
        }
    }
}


