/*
 * Mlflow
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v2.11.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Model20MlflowRegisteredModelsSetTagPostRequest {
    /// Unique name of the model. This field is required.
    #[serde(rename = "name")]
    pub name: String,
    /// Name of the tag. Maximum size depends on storage backend. If a tag with this name already exists, its preexisting value will be replaced by the specified `value`. All storage backends are guaranteed to support key values up to 250 bytes in size. This field is required.
    #[serde(rename = "key")]
    pub key: String,
    /// String value of the tag being logged. Maximum size depends on storage backend. This field is required.
    #[serde(rename = "value")]
    pub value: String,
}

impl Model20MlflowRegisteredModelsSetTagPostRequest {
    pub fn new(name: String, key: String, value: String) -> Model20MlflowRegisteredModelsSetTagPostRequest {
        Model20MlflowRegisteredModelsSetTagPostRequest {
            name,
            key,
            value,
        }
    }
}

