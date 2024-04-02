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
pub struct Modelregistryservicesearchmodelversions200Response {
    /// Models that match the search criteria
    #[serde(rename = "model_versions", skip_serializing_if = "Option::is_none")]
    pub model_versions: Option<Vec<models::Mlflowmodelversion>>,
    /// Pagination token to request next page of models for the same search query.
    #[serde(rename = "next_page_token", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl Modelregistryservicesearchmodelversions200Response {
    pub fn new() -> Modelregistryservicesearchmodelversions200Response {
        Modelregistryservicesearchmodelversions200Response {
            model_versions: None,
            next_page_token: None,
        }
    }
}
