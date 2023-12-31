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
pub struct HaTypeJbsPrimitive {
    #[serde(rename = "primitive")]
    pub primitive: crate::models::HaPrimitive,
    #[serde(rename = "nullable")]
    pub nullable: bool,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

impl HaTypeJbsPrimitive {
    pub fn new(primitive: crate::models::HaPrimitive, nullable: bool, tags: Vec<String>) -> HaTypeJbsPrimitive {
        HaTypeJbsPrimitive {
            primitive,
            nullable,
            tags,
        }
    }
}


