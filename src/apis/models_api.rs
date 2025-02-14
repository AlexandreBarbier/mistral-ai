/*
 * Mistral AI API
 *
 * Our Chat Completion and Embeddings APIs specification. Create your account on [La Plateforme](https://console.mistral.ai) to get access and read the [docs](https://docs.mistral.ai) to learn how to use it.
 *
 * The version of the OpenAPI document: 0.0.2
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`delete_model_v1_models_model_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteModelV1ModelsModelIdDeleteError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`jobs_api_routes_fine_tuning_archive_fine_tuned_model`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JobsApiRoutesFineTuningArchiveFineTunedModelError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`jobs_api_routes_fine_tuning_unarchive_fine_tuned_model`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JobsApiRoutesFineTuningUnarchiveFineTunedModelError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`jobs_api_routes_fine_tuning_update_fine_tuned_model`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JobsApiRoutesFineTuningUpdateFineTunedModelError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_models_v1_models_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListModelsV1ModelsGetError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retrieve_model_v1_models_model_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveModelV1ModelsModelIdGetError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


/// Delete a fine-tuned model.
pub async fn delete_model_v1_models_model_id_delete(configuration: &configuration::Configuration, model_id: &str) -> Result<models::DeleteModelOut, Error<DeleteModelV1ModelsModelIdDeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_model_id = model_id;

    let uri_str = format!("{}/v1/models/{model_id}", configuration.base_path, model_id=crate::apis::urlencode(p_model_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteModelV1ModelsModelIdDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Archive a fine-tuned model.
pub async fn jobs_api_routes_fine_tuning_archive_fine_tuned_model(configuration: &configuration::Configuration, model_id: &str) -> Result<models::ArchiveFtModelOut, Error<JobsApiRoutesFineTuningArchiveFineTunedModelError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_model_id = model_id;

    let uri_str = format!("{}/v1/fine_tuning/models/{model_id}/archive", configuration.base_path, model_id=crate::apis::urlencode(p_model_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<JobsApiRoutesFineTuningArchiveFineTunedModelError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Un-archive a fine-tuned model.
pub async fn jobs_api_routes_fine_tuning_unarchive_fine_tuned_model(configuration: &configuration::Configuration, model_id: &str) -> Result<models::UnarchiveFtModelOut, Error<JobsApiRoutesFineTuningUnarchiveFineTunedModelError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_model_id = model_id;

    let uri_str = format!("{}/v1/fine_tuning/models/{model_id}/archive", configuration.base_path, model_id=crate::apis::urlencode(p_model_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<JobsApiRoutesFineTuningUnarchiveFineTunedModelError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update a model name or description.
pub async fn jobs_api_routes_fine_tuning_update_fine_tuned_model(configuration: &configuration::Configuration, model_id: &str, update_ft_model_in: models::UpdateFtModelIn) -> Result<models::FtModelOut, Error<JobsApiRoutesFineTuningUpdateFineTunedModelError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_model_id = model_id;
    let p_update_ft_model_in = update_ft_model_in;

    let uri_str = format!("{}/v1/fine_tuning/models/{model_id}", configuration.base_path, model_id=crate::apis::urlencode(p_model_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_update_ft_model_in);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<JobsApiRoutesFineTuningUpdateFineTunedModelError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// List all models available to the user.
pub async fn list_models_v1_models_get(configuration: &configuration::Configuration, ) -> Result<models::ModelList, Error<ListModelsV1ModelsGetError>> {

    let uri_str = format!("{}/v1/models", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<ListModelsV1ModelsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve a model information.
pub async fn retrieve_model_v1_models_model_id_get(configuration: &configuration::Configuration, model_id: &str) -> Result<models::ResponseRetrieveModelV1ModelsModelIdGet, Error<RetrieveModelV1ModelsModelIdGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_model_id = model_id;

    let uri_str = format!("{}/v1/models/{model_id}", configuration.base_path, model_id=crate::apis::urlencode(p_model_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<RetrieveModelV1ModelsModelIdGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

