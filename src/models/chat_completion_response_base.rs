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
pub struct ChatCompletionResponseBase {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Box<models::UsageInfo>>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i32>,
}

impl ChatCompletionResponseBase {
    pub fn new() -> ChatCompletionResponseBase {
        ChatCompletionResponseBase {
            id: None,
            object: None,
            model: None,
            usage: None,
            created: None,
        }
    }
}

