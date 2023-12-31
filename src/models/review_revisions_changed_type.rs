/*
 * Space
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023.3.0-DEV.171131
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReviewRevisionsChangedType {
    #[serde(rename = "Created")]
    Created,
    #[serde(rename = "Added")]
    Added,
    #[serde(rename = "Removed")]
    Removed,

}

impl ToString for ReviewRevisionsChangedType {
    fn to_string(&self) -> String {
        match self {
            Self::Created => String::from("Created"),
            Self::Added => String::from("Added"),
            Self::Removed => String::from("Removed"),
        }
    }
}

impl Default for ReviewRevisionsChangedType {
    fn default() -> ReviewRevisionsChangedType {
        Self::Created
    }
}




