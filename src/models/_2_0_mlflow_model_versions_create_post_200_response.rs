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
pub struct Model20MlflowModelVersionsCreatePost200Response {
    #[serde(rename = "model_version", skip_serializing_if = "Option::is_none")]
    pub model_version: Option<Box<models::Mlflowmodelversion>>,
}

impl Model20MlflowModelVersionsCreatePost200Response {
    pub fn new() -> Model20MlflowModelVersionsCreatePost200Response {
        Model20MlflowModelVersionsCreatePost200Response {
            model_version: None,
        }
    }
}

