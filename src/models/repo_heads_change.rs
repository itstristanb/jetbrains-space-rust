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
pub struct RepoHeadsChange {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "oldId")]
    pub old_id: String,
    #[serde(rename = "newId")]
    pub new_id: String,
    #[serde(rename = "force", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub force: Option<Option<bool>>,
}

impl RepoHeadsChange {
    pub fn new(name: String, old_id: String, new_id: String) -> RepoHeadsChange {
        RepoHeadsChange {
            name,
            old_id,
            new_id,
            force: None,
        }
    }
}

