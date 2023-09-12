/*
 * FastAPI
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LocateBucketResponse {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "cloud")]
    pub cloud: String,
    #[serde(rename = "bucket")]
    pub bucket: String,
    #[serde(rename = "region")]
    pub region: String,
}

impl LocateBucketResponse {
    pub fn new(
        id: i32,
        tag: String,
        cloud: String,
        bucket: String,
        region: String,
    ) -> LocateBucketResponse {
        LocateBucketResponse {
            id,
            tag,
            cloud,
            bucket,
            region,
        }
    }
}
