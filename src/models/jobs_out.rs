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
pub struct JobsOut {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::JobOut>>,
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<Object>,
    #[serde(rename = "total")]
    pub total: i32,
}

impl JobsOut {
    pub fn new(total: i32) -> JobsOut {
        JobsOut {
            data: None,
            object: None,
            total,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "list")]
    List,
}

impl Default for Object {
    fn default() -> Object {
        Self::List
    }
}

