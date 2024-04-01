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
pub struct Model20MlflowRunsLogBatchPostRequest {
    #[serde(rename = "run_id", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<models::Mlflowmetric>>,
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<models::Mlflowparam>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<models::Mlflowruntag>>,
}

impl Model20MlflowRunsLogBatchPostRequest {
    pub fn new() -> Model20MlflowRunsLogBatchPostRequest {
        Model20MlflowRunsLogBatchPostRequest {
            run_id: None,
            metrics: None,
            params: None,
            tags: None,
        }
    }
}
