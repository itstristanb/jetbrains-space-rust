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
pub struct PrTag {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "projectCount")]
    pub project_count: i32,
}

impl PrTag {
    pub fn new(name: String, project_count: i32) -> PrTag {
        PrTag {
            name,
            project_count,
        }
    }
}


