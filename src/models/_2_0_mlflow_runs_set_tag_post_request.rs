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
pub struct Model20MlflowRunsSetTagPostRequest {
    #[serde(rename = "run_id", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "run_uuid", skip_serializing_if = "Option::is_none")]
    pub run_uuid: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Model20MlflowRunsSetTagPostRequest {
    pub fn new() -> Model20MlflowRunsSetTagPostRequest {
        Model20MlflowRunsSetTagPostRequest {
            run_id: None,
            run_uuid: None,
            key: None,
            value: None,
        }
    }
}

