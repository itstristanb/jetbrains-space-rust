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
pub enum SpecialDateValue {
    #[serde(rename = "NOT_SET")]
    NotSet,
    #[serde(rename = "TODAY")]
    Today,
    #[serde(rename = "YESTERDAY")]
    Yesterday,
    #[serde(rename = "NEXT_WEEK")]
    NextWeek,
    #[serde(rename = "THIS_WEEK")]
    ThisWeek,
    #[serde(rename = "LAST_WEEK")]
    LastWeek,

}

impl ToString for SpecialDateValue {
    fn to_string(&self) -> String {
        match self {
            Self::NotSet => String::from("NOT_SET"),
            Self::Today => String::from("TODAY"),
            Self::Yesterday => String::from("YESTERDAY"),
            Self::NextWeek => String::from("NEXT_WEEK"),
            Self::ThisWeek => String::from("THIS_WEEK"),
            Self::LastWeek => String::from("LAST_WEEK"),
        }
    }
}

impl Default for SpecialDateValue {
    fn default() -> SpecialDateValue {
        Self::NotSet
    }
}



