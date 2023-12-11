/*
 * FastAPI
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationError {
    #[serde(rename = "loc")]
    pub loc: Vec<crate::models::LocationInner>,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl ValidationError {
    pub fn new(loc: Vec<crate::models::LocationInner>, msg: String, r#type: String) -> ValidationError {
        ValidationError {
            loc,
            msg,
            r#type,
        }
    }
}


