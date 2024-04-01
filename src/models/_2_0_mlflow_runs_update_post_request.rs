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
pub struct Model20MlflowRunsUpdatePostRequest {
    #[serde(rename = "run_id", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "run_uuid", skip_serializing_if = "Option::is_none")]
    pub run_uuid: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::Mlflowrunstatus>,
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(rename = "run_name", skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,
}

impl Model20MlflowRunsUpdatePostRequest {
    pub fn new() -> Model20MlflowRunsUpdatePostRequest {
        Model20MlflowRunsUpdatePostRequest {
            run_id: None,
            run_uuid: None,
            status: None,
            end_time: None,
            run_name: None,
        }
    }
}

