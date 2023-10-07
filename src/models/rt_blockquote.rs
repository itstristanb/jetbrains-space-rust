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
pub struct RtBlockquote {
    #[serde(rename = "children")]
    pub children: Vec<crate::models::RtBlockNode>,
}

impl RtBlockquote {
    pub fn new(children: Vec<crate::models::RtBlockNode>) -> RtBlockquote {
        RtBlockquote {
            children,
        }
    }
}


