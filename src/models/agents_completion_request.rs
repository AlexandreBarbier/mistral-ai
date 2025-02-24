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
pub struct AgentsCompletionRequest {
    #[serde(
        rename = "max_tokens",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_tokens: Option<Option<i32>>,
    /// Whether to stream back partial progress. If set, tokens will be sent as data-only server-side events as they become available, with the stream terminated by a data: \[DONE\] message. Otherwise, the server will hold the request open until the timeout or until completion, with the response containing the full result as JSON.
    #[serde(rename = "stream", skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(rename = "stop", skip_serializing_if = "Option::is_none")]
    pub stop: Option<Box<models::Stop>>,
    #[serde(
        rename = "random_seed",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub random_seed: Option<Option<i32>>,
    /// The prompt(s) to generate completions for, encoded as a list of dict with role and content.
    #[serde(rename = "messages")]
    pub messages: Vec<models::ChatCompletionRequestMessagesInner>,
    #[serde(rename = "response_format", skip_serializing_if = "Option::is_none")]
    pub response_format: Option<models::ResponseFormat>,
    #[serde(
        rename = "tools",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tools: Option<Option<Vec<models::Tool>>>,
    #[serde(rename = "tool_choice", skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<Box<models::ToolChoice>>,
    /// presence_penalty determines how much the model penalizes the repetition of words or phrases. A higher presence penalty encourages the model to use a wider variety of words and phrases, making the output more diverse and creative.
    #[serde(rename = "presence_penalty", skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    /// frequency_penalty penalizes the repetition of words based on their frequency in the generated text. A higher frequency penalty discourages the model from repeating words that have already appeared frequently in the output, promoting diversity and reducing repetition.
    #[serde(rename = "frequency_penalty", skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(
        rename = "n",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub n: Option<Option<i32>>,
    /// Enable users to specify expected results, optimizing response times by leveraging known or predictable content. This approach is especially effective for updating text documents or code files with minimal changes, reducing latency while maintaining high-quality results.
    #[serde(rename = "prediction", skip_serializing_if = "Option::is_none")]
    pub prediction: Option<models::Prediction>,
    /// The ID of the agent to use for this completion.
    #[serde(rename = "agent_id")]
    pub agent_id: String,
}

impl AgentsCompletionRequest {
    pub fn new(
        messages: Vec<models::ChatCompletionRequestMessagesInner>,
        agent_id: String,
    ) -> AgentsCompletionRequest {
        AgentsCompletionRequest {
            max_tokens: None,
            stream: None,
            stop: None,
            random_seed: None,
            messages,
            response_format: None,
            tools: None,
            tool_choice: None,
            presence_penalty: None,
            frequency_penalty: None,
            n: None,
            prediction: None,
            agent_id,
        }
    }
}
