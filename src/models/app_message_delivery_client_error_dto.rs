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
pub struct AppMessageDeliveryClientErrorDto {
    #[serde(rename = "className")]
    pub class_name: String,
    #[serde(rename = "message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub message: Option<Option<String>>,
    #[serde(rename = "causeClassName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cause_class_name: Option<Option<String>>,
    #[serde(rename = "causeMessage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cause_message: Option<Option<String>>,
}

impl AppMessageDeliveryClientErrorDto {
    pub fn new(class_name: String) -> AppMessageDeliveryClientErrorDto {
        AppMessageDeliveryClientErrorDto {
            class_name,
            message: None,
            cause_class_name: None,
            cause_message: None,
        }
    }
}


