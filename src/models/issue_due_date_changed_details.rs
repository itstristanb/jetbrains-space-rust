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
pub struct IssueDueDateChangedDetails {
    #[serde(rename = "oldDueDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub old_due_date: Option<Option<String>>,
    #[serde(rename = "newDueDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub new_due_date: Option<Option<String>>,
}

impl IssueDueDateChangedDetails {
    pub fn new() -> IssueDueDateChangedDetails {
        IssueDueDateChangedDetails {
            old_due_date: None,
            new_due_date: None,
        }
    }
}


