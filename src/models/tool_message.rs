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
pub struct ToolMessage {
    #[serde(rename = "content", deserialize_with = "Option::deserialize")]
    pub content: Option<Box<models::Content>>,
    #[serde(rename = "tool_call_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<Option<String>>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

impl ToolMessage {
    pub fn new(content: Option<models::Content>) -> ToolMessage {
        ToolMessage {
            content: if let Some(x) = content {Some(Box::new(x))} else {None},
            tool_call_id: None,
            name: None,
            role: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "tool")]
    Tool,
}

impl Default for Role {
    fn default() -> Role {
        Self::Tool
    }
}

