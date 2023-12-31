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
pub struct IssueTitleChangedDetails {
    #[serde(rename = "oldTitle")]
    pub old_title: String,
    #[serde(rename = "newTitle")]
    pub new_title: String,
}

impl IssueTitleChangedDetails {
    pub fn new(old_title: String, new_title: String) -> IssueTitleChangedDetails {
        IssueTitleChangedDetails {
            old_title,
            new_title,
        }
    }
}


