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
pub struct SetPolicyRequest {
    #[serde(rename = "get_policy", skip_serializing_if = "Option::is_none")]
    pub get_policy: Option<String>,
    #[serde(rename = "put_policy", skip_serializing_if = "Option::is_none")]
    pub put_policy: Option<String>,
}

impl Default for SetPolicyRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl SetPolicyRequest {
    pub fn new() -> SetPolicyRequest {
        SetPolicyRequest {
            get_policy: None,
            put_policy: None,
        }
    }
}
