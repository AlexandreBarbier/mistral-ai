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

/// FtModelCard : Extra fields for fine-tuned models.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FtModelCard {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i32>,
    #[serde(rename = "owned_by", skip_serializing_if = "Option::is_none")]
    pub owned_by: Option<String>,
    #[serde(rename = "capabilities")]
    pub capabilities: Box<models::ModelCapabilities>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "max_context_length", skip_serializing_if = "Option::is_none")]
    pub max_context_length: Option<i32>,
    #[serde(rename = "aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "deprecation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deprecation: Option<Option<String>>,
    #[serde(rename = "default_model_temperature", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_model_temperature: Option<Option<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "job")]
    pub job: String,
    #[serde(rename = "root")]
    pub root: String,
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}

impl FtModelCard {
    /// Extra fields for fine-tuned models.
    pub fn new(id: String, capabilities: models::ModelCapabilities, job: String, root: String) -> FtModelCard {
        FtModelCard {
            id,
            object: None,
            created: None,
            owned_by: None,
            capabilities: Box::new(capabilities),
            name: None,
            description: None,
            max_context_length: None,
            aliases: None,
            deprecation: None,
            default_model_temperature: None,
            r#type: None,
            job,
            root,
            archived: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "fine-tuned")]
    FineTuned,
}

impl Default for Type {
    fn default() -> Type {
        Self::FineTuned
    }
}

