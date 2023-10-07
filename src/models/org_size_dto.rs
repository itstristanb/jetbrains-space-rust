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
pub enum OrgSizeDto {
    #[serde(rename = "JUST_ME")]
    JustMe,
    #[serde(rename = "N_2_10")]
    N210,
    #[serde(rename = "N_11_50")]
    N1150,
    #[serde(rename = "N_51_250")]
    N51250,
    #[serde(rename = "N_251_500")]
    N251500,
    #[serde(rename = "N_501_1000")]
    N5011000,
    #[serde(rename = "N_1001_5000")]
    N10015000,
    #[serde(rename = "MORE_THAN_5000")]
    MoreThan5000,

}

impl ToString for OrgSizeDto {
    fn to_string(&self) -> String {
        match self {
            Self::JustMe => String::from("JUST_ME"),
            Self::N210 => String::from("N_2_10"),
            Self::N1150 => String::from("N_11_50"),
            Self::N51250 => String::from("N_51_250"),
            Self::N251500 => String::from("N_251_500"),
            Self::N5011000 => String::from("N_501_1000"),
            Self::N10015000 => String::from("N_1001_5000"),
            Self::MoreThan5000 => String::from("MORE_THAN_5000"),
        }
    }
}

impl Default for OrgSizeDto {
    fn default() -> OrgSizeDto {
        Self::JustMe
    }
}




