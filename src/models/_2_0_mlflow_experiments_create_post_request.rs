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
pub struct Model20MlflowExperimentsCreatePostRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "artifact_location", skip_serializing_if = "Option::is_none")]
    pub artifact_location: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<models::Mlflowexperimenttag>>,
}

impl Model20MlflowExperimentsCreatePostRequest {
    pub fn new() -> Model20MlflowExperimentsCreatePostRequest {
        Model20MlflowExperimentsCreatePostRequest {
            name: None,
            artifact_location: None,
            tags: None,
        }
    }
}

