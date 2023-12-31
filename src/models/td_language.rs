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
pub struct TdLanguage {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nativeName")]
    pub native_name: String,
    #[serde(rename = "firstNameTitle")]
    pub first_name_title: String,
    #[serde(rename = "lastNameTitle")]
    pub last_name_title: String,
}

impl TdLanguage {
    pub fn new(id: String, code: String, name: String, native_name: String, first_name_title: String, last_name_title: String) -> TdLanguage {
        TdLanguage {
            id,
            code,
            name,
            native_name,
            first_name_title,
            last_name_title,
        }
    }
}


