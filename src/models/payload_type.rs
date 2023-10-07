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
pub enum PayloadType {
    #[serde(rename = "FIELD_SELECTION")]
    FieldSelection,
    #[serde(rename = "STRING_TEMPLATE")]
    StringTemplate,

}

impl ToString for PayloadType {
    fn to_string(&self) -> String {
        match self {
            Self::FieldSelection => String::from("FIELD_SELECTION"),
            Self::StringTemplate => String::from("STRING_TEMPLATE"),
        }
    }
}

impl Default for PayloadType {
    fn default() -> PayloadType {
        Self::FieldSelection
    }
}



