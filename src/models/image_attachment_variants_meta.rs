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
pub struct ImageAttachmentVariantsMeta {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "width")]
    pub width: i32,
    #[serde(rename = "height")]
    pub height: i32,
}

impl ImageAttachmentVariantsMeta {
    pub fn new(id: String, name: String, width: i32, height: i32) -> ImageAttachmentVariantsMeta {
        ImageAttachmentVariantsMeta {
            id,
            name,
            width,
            height,
        }
    }
}


