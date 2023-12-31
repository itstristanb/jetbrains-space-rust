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
pub struct RtDocument {
    #[serde(rename = "children")]
    pub children: Vec<crate::models::RtBlockNode>,
    #[serde(rename = "version")]
    pub version: String,
}

impl RtDocument {
    pub fn new(children: Vec<crate::models::RtBlockNode>, version: String) -> RtDocument {
        RtDocument {
            children,
            version,
        }
    }
}


