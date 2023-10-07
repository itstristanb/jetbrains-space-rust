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
pub struct HaTypeJbsMap {
    #[serde(rename = "valueType")]
    pub value_type: Box<crate::models::HaType>,
    #[serde(rename = "nullable")]
    pub nullable: bool,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

impl HaTypeJbsMap {
    pub fn new(value_type: crate::models::HaType, nullable: bool, tags: Vec<String>) -> HaTypeJbsMap {
        HaTypeJbsMap {
            value_type: Box::new(value_type),
            nullable,
            tags,
        }
    }
}


