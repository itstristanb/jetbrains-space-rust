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
pub struct RtHeading {
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "children")]
    pub children: Vec<crate::models::RtHeadingContentNode>,
    #[serde(rename = "textAlign")]
    pub text_align: crate::models::RtTextAlign,
}

impl RtHeading {
    pub fn new(level: i32, children: Vec<crate::models::RtHeadingContentNode>, text_align: crate::models::RtTextAlign) -> RtHeading {
        RtHeading {
            level,
            children,
            text_align,
        }
    }
}


