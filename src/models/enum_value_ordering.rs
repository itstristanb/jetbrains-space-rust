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
pub enum EnumValueOrdering {
    #[serde(rename = "NAME_ASC")]
    Asc,
    #[serde(rename = "NAME_DESC")]
    Desc,

}

impl ToString for EnumValueOrdering {
    fn to_string(&self) -> String {
        match self {
            Self::Asc => String::from("NAME_ASC"),
            Self::Desc => String::from("NAME_DESC"),
        }
    }
}

impl Default for EnumValueOrdering {
    fn default() -> EnumValueOrdering {
        Self::Asc
    }
}




