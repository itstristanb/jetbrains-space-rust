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
pub enum CustomFieldType {
    #[serde(rename = "AUTONUMBER")]
    Autonumber,
    #[serde(rename = "BOOLEAN")]
    Boolean,
    #[serde(rename = "DATE")]
    Date,
    #[serde(rename = "DATE_TIME")]
    DateTime,
    #[serde(rename = "DOCUMENT")]
    Document,
    #[serde(rename = "ENUM")]
    Enum,
    #[serde(rename = "INTEGER")]
    Integer,
    #[serde(rename = "ISSUE")]
    Issue,
    #[serde(rename = "LOCATION")]
    Location,
    #[serde(rename = "OPEN_ENUM")]
    OpenEnum,
    #[serde(rename = "PERCENTAGE")]
    Percentage,
    #[serde(rename = "PROFILE")]
    Profile,
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "STRING")]
    String,
    #[serde(rename = "TEAM")]
    Team,
    #[serde(rename = "URL")]
    Url,
    #[serde(rename = "COMMIT")]
    Commit,

}

impl ToString for CustomFieldType {
    fn to_string(&self) -> String {
        match self {
            Self::Autonumber => String::from("AUTONUMBER"),
            Self::Boolean => String::from("BOOLEAN"),
            Self::Date => String::from("DATE"),
            Self::DateTime => String::from("DATE_TIME"),
            Self::Document => String::from("DOCUMENT"),
            Self::Enum => String::from("ENUM"),
            Self::Integer => String::from("INTEGER"),
            Self::Issue => String::from("ISSUE"),
            Self::Location => String::from("LOCATION"),
            Self::OpenEnum => String::from("OPEN_ENUM"),
            Self::Percentage => String::from("PERCENTAGE"),
            Self::Profile => String::from("PROFILE"),
            Self::Project => String::from("PROJECT"),
            Self::String => String::from("STRING"),
            Self::Team => String::from("TEAM"),
            Self::Url => String::from("URL"),
            Self::Commit => String::from("COMMIT"),
        }
    }
}

impl Default for CustomFieldType {
    fn default() -> CustomFieldType {
        Self::Autonumber
    }
}



