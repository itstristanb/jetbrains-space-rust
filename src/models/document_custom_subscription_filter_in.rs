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
pub struct DocumentCustomSubscriptionFilterIn {
    #[serde(rename = "project", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project: Option<Option<String>>,
    #[serde(rename = "folders", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub folders: Option<Option<Vec<String>>>,
    #[serde(rename = "documents")]
    pub documents: Vec<String>,
}

impl DocumentCustomSubscriptionFilterIn {
    pub fn new(documents: Vec<String>) -> DocumentCustomSubscriptionFilterIn {
        DocumentCustomSubscriptionFilterIn {
            project: None,
            folders: None,
            documents,
        }
    }
}


