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
pub struct DeltaMessage {
    #[serde(rename = "role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role: Option<Option<String>>,
    #[serde(rename = "content", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content: Option<Option<Box<models::DeltaMessageContent>>>,
    #[serde(rename = "tool_calls", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Option<Vec<models::ToolCall>>>,
}

impl DeltaMessage {
    pub fn new() -> DeltaMessage {
        DeltaMessage {
            role: None,
            content: None,
            tool_calls: None,
        }
    }
}

