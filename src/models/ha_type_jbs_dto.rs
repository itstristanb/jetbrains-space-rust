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
pub struct HaTypeJbsDto {
    #[serde(rename = "dto")]
    pub dto: Box<crate::models::HaDto>,
    #[serde(rename = "nullable")]
    pub nullable: bool,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

impl HaTypeJbsDto {
    pub fn new(dto: crate::models::HaDto, nullable: bool, tags: Vec<String>) -> HaTypeJbsDto {
        HaTypeJbsDto {
            dto: Box::new(dto),
            nullable,
            tags,
        }
    }
}


