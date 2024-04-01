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
pub struct Model20MlflowArtifactsListGet200Response {
    #[serde(rename = "root_uri", skip_serializing_if = "Option::is_none")]
    pub root_uri: Option<String>,
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<models::Mlflowfileinfo>>,
    #[serde(rename = "next_page_token", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl Model20MlflowArtifactsListGet200Response {
    pub fn new() -> Model20MlflowArtifactsListGet200Response {
        Model20MlflowArtifactsListGet200Response {
            root_uri: None,
            files: None,
            next_page_token: None,
        }
    }
}

