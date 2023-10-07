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
pub enum ColumnSortOrder {
    #[serde(rename = "ASC")]
    Asc,
    #[serde(rename = "DESC")]
    Desc,

}

impl ToString for ColumnSortOrder {
    fn to_string(&self) -> String {
        match self {
            Self::Asc => String::from("ASC"),
            Self::Desc => String::from("DESC"),
        }
    }
}

impl Default for ColumnSortOrder {
    fn default() -> ColumnSortOrder {
        Self::Asc
    }
}



