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
pub enum HaPrimitive {
    #[serde(rename = "Byte")]
    Byte,
    #[serde(rename = "Short")]
    Short,
    #[serde(rename = "Int")]
    Int,
    #[serde(rename = "Long")]
    Long,
    #[serde(rename = "Float")]
    Float,
    #[serde(rename = "Double")]
    Double,
    #[serde(rename = "Boolean")]
    Boolean,
    #[serde(rename = "String")]
    String,
    #[serde(rename = "Date")]
    Date,
    #[serde(rename = "DateTime")]
    DateTime,
    #[serde(rename = "Duration")]
    Duration,

}

impl ToString for HaPrimitive {
    fn to_string(&self) -> String {
        match self {
            Self::Byte => String::from("Byte"),
            Self::Short => String::from("Short"),
            Self::Int => String::from("Int"),
            Self::Long => String::from("Long"),
            Self::Float => String::from("Float"),
            Self::Double => String::from("Double"),
            Self::Boolean => String::from("Boolean"),
            Self::String => String::from("String"),
            Self::Date => String::from("Date"),
            Self::DateTime => String::from("DateTime"),
            Self::Duration => String::from("Duration"),
        }
    }
}

impl Default for HaPrimitive {
    fn default() -> HaPrimitive {
        Self::Byte
    }
}




