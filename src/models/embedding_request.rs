/*
 * Mistral AI API
 *
 * Our Chat Completion and Embeddings APIs specification. Create your account on [La Plateforme](https://console.mistral.ai) to get access and read the [docs](https://docs.mistral.ai) to learn how to use it.
 *
 * The version of the OpenAPI document: 0.0.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    #[serde(rename = "input")]
    pub input: Box<models::Input>,
    /// ID of the model to use.
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "encoding_format", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<Option<String>>,
}

impl EmbeddingRequest {
    pub fn new(input: models::Input, model: String) -> EmbeddingRequest {
        EmbeddingRequest {
            input: Box::new(input),
            model,
            encoding_format: None,
        }
    }
}

