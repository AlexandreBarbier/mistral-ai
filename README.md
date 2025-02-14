# Rust API client for openapi

Our Chat Completion and Embeddings APIs specification. Create your account on [La Plateforme](https://console.mistral.ai) to get access and read the [docs](https://docs.mistral.ai) to learn how to use it.

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project. By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.0.2
- Package version: 0.0.2
- Generator version: 7.11.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `mistral-ai` and add the following to `Cargo.toml` under `[dependencies]`:

```
mistral-ai = { path = "./mistral-ai" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.mistral.ai*

| Class            | Method                                                                                                                                 | HTTP request                                         | Description                |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- | -------------------------- |
| _AgentsApi_      | [**agents_completion_v1_agents_completions_post**](docs/AgentsApi.md#agents_completion_v1_agents_completions_post)                     | **POST** /v1/agents/completions                      | Agents Completion          |
| _BatchApi_       | [**jobs_api_routes_batch_cancel_batch_job**](docs/BatchApi.md#jobs_api_routes_batch_cancel_batch_job)                                  | **POST** /v1/batch/jobs/{job_id}/cancel              | Cancel Batch Job           |
| _BatchApi_       | [**jobs_api_routes_batch_create_batch_job**](docs/BatchApi.md#jobs_api_routes_batch_create_batch_job)                                  | **POST** /v1/batch/jobs                              | Create Batch Job           |
| _BatchApi_       | [**jobs_api_routes_batch_get_batch_job**](docs/BatchApi.md#jobs_api_routes_batch_get_batch_job)                                        | **GET** /v1/batch/jobs/{job_id}                      | Get Batch Job              |
| _BatchApi_       | [**jobs_api_routes_batch_get_batch_jobs**](docs/BatchApi.md#jobs_api_routes_batch_get_batch_jobs)                                      | **GET** /v1/batch/jobs                               | Get Batch Jobs             |
| _ChatApi_        | [**chat_completion_v1_chat_completions_post**](docs/ChatApi.md#chat_completion_v1_chat_completions_post)                               | **POST** /v1/chat/completions                        | Chat Completion            |
| _ClassifiersApi_ | [**moderations_chat_v1_chat_moderations_post**](docs/ClassifiersApi.md#moderations_chat_v1_chat_moderations_post)                      | **POST** /v1/chat/moderations                        | Moderations Chat           |
| _ClassifiersApi_ | [**moderations_v1_moderations_post**](docs/ClassifiersApi.md#moderations_v1_moderations_post)                                          | **POST** /v1/moderations                             | Moderations                |
| _EmbeddingsApi_  | [**embeddings_v1_embeddings_post**](docs/EmbeddingsApi.md#embeddings_v1_embeddings_post)                                               | **POST** /v1/embeddings                              | Embeddings                 |
| _FilesApi_       | [**files_api_routes_delete_file**](docs/FilesApi.md#files_api_routes_delete_file)                                                      | **DELETE** /v1/files/{file_id}                       | Delete File                |
| _FilesApi_       | [**files_api_routes_download_file**](docs/FilesApi.md#files_api_routes_download_file)                                                  | **GET** /v1/files/{file_id}/content                  | Download File              |
| _FilesApi_       | [**files_api_routes_get_signed_url**](docs/FilesApi.md#files_api_routes_get_signed_url)                                                | **GET** /v1/files/{file_id}/url                      | Get Signed Url             |
| _FilesApi_       | [**files_api_routes_list_files**](docs/FilesApi.md#files_api_routes_list_files)                                                        | **GET** /v1/files                                    | List Files                 |
| _FilesApi_       | [**files_api_routes_retrieve_file**](docs/FilesApi.md#files_api_routes_retrieve_file)                                                  | **GET** /v1/files/{file_id}                          | Retrieve File              |
| _FilesApi_       | [**files_api_routes_upload_file**](docs/FilesApi.md#files_api_routes_upload_file)                                                      | **POST** /v1/files                                   | Upload File                |
| _FimApi_         | [**fim_completion_v1_fim_completions_post**](docs/FimApi.md#fim_completion_v1_fim_completions_post)                                    | **POST** /v1/fim/completions                         | Fim Completion             |
| _FineTuningApi_  | [**jobs_api_routes_fine_tuning_cancel_fine_tuning_job**](docs/FineTuningApi.md#jobs_api_routes_fine_tuning_cancel_fine_tuning_job)     | **POST** /v1/fine_tuning/jobs/{job_id}/cancel        | Cancel Fine Tuning Job     |
| _FineTuningApi_  | [**jobs_api_routes_fine_tuning_create_fine_tuning_job**](docs/FineTuningApi.md#jobs_api_routes_fine_tuning_create_fine_tuning_job)     | **POST** /v1/fine_tuning/jobs                        | Create Fine Tuning Job     |
| _FineTuningApi_  | [**jobs_api_routes_fine_tuning_get_fine_tuning_job**](docs/FineTuningApi.md#jobs_api_routes_fine_tuning_get_fine_tuning_job)           | **GET** /v1/fine_tuning/jobs/{job_id}                | Get Fine Tuning Job        |
| _FineTuningApi_  | [**jobs_api_routes_fine_tuning_get_fine_tuning_jobs**](docs/FineTuningApi.md#jobs_api_routes_fine_tuning_get_fine_tuning_jobs)         | **GET** /v1/fine_tuning/jobs                         | Get Fine Tuning Jobs       |
| _FineTuningApi_  | [**jobs_api_routes_fine_tuning_start_fine_tuning_job**](docs/FineTuningApi.md#jobs_api_routes_fine_tuning_start_fine_tuning_job)       | **POST** /v1/fine_tuning/jobs/{job_id}/start         | Start Fine Tuning Job      |
| _ModelsApi_      | [**delete_model_v1_models_model_id_delete**](docs/ModelsApi.md#delete_model_v1_models_model_id_delete)                                 | **DELETE** /v1/models/{model_id}                     | Delete Model               |
| _ModelsApi_      | [**jobs_api_routes_fine_tuning_archive_fine_tuned_model**](docs/ModelsApi.md#jobs_api_routes_fine_tuning_archive_fine_tuned_model)     | **POST** /v1/fine_tuning/models/{model_id}/archive   | Archive Fine Tuned Model   |
| _ModelsApi_      | [**jobs_api_routes_fine_tuning_unarchive_fine_tuned_model**](docs/ModelsApi.md#jobs_api_routes_fine_tuning_unarchive_fine_tuned_model) | **DELETE** /v1/fine_tuning/models/{model_id}/archive | Unarchive Fine Tuned Model |
| _ModelsApi_      | [**jobs_api_routes_fine_tuning_update_fine_tuned_model**](docs/ModelsApi.md#jobs_api_routes_fine_tuning_update_fine_tuned_model)       | **PATCH** /v1/fine_tuning/models/{model_id}          | Update Fine Tuned Model    |
| _ModelsApi_      | [**list_models_v1_models_get**](docs/ModelsApi.md#list_models_v1_models_get)                                                           | **GET** /v1/models                                   | List Models                |
| _ModelsApi_      | [**retrieve_model_v1_models_model_id_get**](docs/ModelsApi.md#retrieve_model_v1_models_model_id_get)                                   | **GET** /v1/models/{model_id}                        | Retrieve Model             |

## Documentation For Models

- [AgentsCompletionRequest](docs/AgentsCompletionRequest.md)
- [ApiEndpoint](docs/ApiEndpoint.md)
- [ArchiveFtModelOut](docs/ArchiveFtModelOut.md)
- [Arguments](docs/Arguments.md)
- [AssistantMessage](docs/AssistantMessage.md)
- [BaseModelCard](docs/BaseModelCard.md)
- [BatchError](docs/BatchError.md)
- [BatchJobIn](docs/BatchJobIn.md)
- [BatchJobOut](docs/BatchJobOut.md)
- [BatchJobStatus](docs/BatchJobStatus.md)
- [BatchJobsOut](docs/BatchJobsOut.md)
- [ChatClassificationRequest](docs/ChatClassificationRequest.md)
- [ChatCompletionChoice](docs/ChatCompletionChoice.md)
- [ChatCompletionRequest](docs/ChatCompletionRequest.md)
- [ChatCompletionRequestMessagesInner](docs/ChatCompletionRequestMessagesInner.md)
- [ChatCompletionResponse](docs/ChatCompletionResponse.md)
- [ChatCompletionResponseBase](docs/ChatCompletionResponseBase.md)
- [CheckpointOut](docs/CheckpointOut.md)
- [ClassificationObject](docs/ClassificationObject.md)
- [ClassificationRequest](docs/ClassificationRequest.md)
- [ClassificationResponse](docs/ClassificationResponse.md)
- [CompletionChunk](docs/CompletionChunk.md)
- [CompletionEvent](docs/CompletionEvent.md)
- [CompletionResponseStreamChoice](docs/CompletionResponseStreamChoice.md)
- [Content](docs/Content.md)
- [Content1](docs/Content1.md)
- [ContentChunk](docs/ContentChunk.md)
- [DeleteFileOut](docs/DeleteFileOut.md)
- [DeleteModelOut](docs/DeleteModelOut.md)
- [DeltaMessage](docs/DeltaMessage.md)
- [DeltaMessageContent](docs/DeltaMessageContent.md)
- [DetailedJobOut](docs/DetailedJobOut.md)
- [EmbeddingRequest](docs/EmbeddingRequest.md)
- [EmbeddingResponse](docs/EmbeddingResponse.md)
- [EmbeddingResponseData](docs/EmbeddingResponseData.md)
- [EventOut](docs/EventOut.md)
- [FilePurpose](docs/FilePurpose.md)
- [FileSchema](docs/FileSchema.md)
- [FileSignedUrl](docs/FileSignedUrl.md)
- [FimCompletionRequest](docs/FimCompletionRequest.md)
- [FimCompletionResponse](docs/FimCompletionResponse.md)
- [FineTuneableModel](docs/FineTuneableModel.md)
- [FtModelCapabilitiesOut](docs/FtModelCapabilitiesOut.md)
- [FtModelCard](docs/FtModelCard.md)
- [FtModelOut](docs/FtModelOut.md)
- [Function](docs/Function.md)
- [FunctionCall](docs/FunctionCall.md)
- [FunctionName](docs/FunctionName.md)
- [GithubRepositoryIn](docs/GithubRepositoryIn.md)
- [GithubRepositoryOut](docs/GithubRepositoryOut.md)
- [HttpValidationError](docs/HttpValidationError.md)
- [ImageUrl](docs/ImageUrl.md)
- [ImageUrlChunk](docs/ImageUrlChunk.md)
- [Input](docs/Input.md)
- [Input1](docs/Input1.md)
- [Input2](docs/Input2.md)
- [JobIn](docs/JobIn.md)
- [JobInIntegrationsInner](docs/JobInIntegrationsInner.md)
- [JobInRepositoriesInner](docs/JobInRepositoriesInner.md)
- [JobMetadataOut](docs/JobMetadataOut.md)
- [JobOut](docs/JobOut.md)
- [JobOutIntegrationsInner](docs/JobOutIntegrationsInner.md)
- [JobOutRepositoriesInner](docs/JobOutRepositoriesInner.md)
- [JobsOut](docs/JobsOut.md)
- [LegacyJobMetadataOut](docs/LegacyJobMetadataOut.md)
- [ListFilesOut](docs/ListFilesOut.md)
- [MetricOut](docs/MetricOut.md)
- [ModelCapabilities](docs/ModelCapabilities.md)
- [ModelList](docs/ModelList.md)
- [ModelListDataInner](docs/ModelListDataInner.md)
- [Prediction](docs/Prediction.md)
- [ReferenceChunk](docs/ReferenceChunk.md)
- [Response](docs/Response.md)
- [ResponseBase](docs/ResponseBase.md)
- [ResponseFormat](docs/ResponseFormat.md)
- [ResponseFormats](docs/ResponseFormats.md)
- [ResponseRetrieveModelV1ModelsModelIdGet](docs/ResponseRetrieveModelV1ModelsModelIdGet.md)
- [RetrieveFileOut](docs/RetrieveFileOut.md)
- [SampleType](docs/SampleType.md)
- [Source](docs/Source.md)
- [Stop](docs/Stop.md)
- [SystemMessage](docs/SystemMessage.md)
- [TextChunk](docs/TextChunk.md)
- [Tool](docs/Tool.md)
- [ToolCall](docs/ToolCall.md)
- [ToolChoice](docs/ToolChoice.md)
- [ToolChoiceEnum](docs/ToolChoiceEnum.md)
- [ToolMessage](docs/ToolMessage.md)
- [ToolTypes](docs/ToolTypes.md)
- [TrainingFile](docs/TrainingFile.md)
- [TrainingParameters](docs/TrainingParameters.md)
- [TrainingParametersIn](docs/TrainingParametersIn.md)
- [UnarchiveFtModelOut](docs/UnarchiveFtModelOut.md)
- [UpdateFtModelIn](docs/UpdateFtModelIn.md)
- [UploadFileOut](docs/UploadFileOut.md)
- [UsageInfo](docs/UsageInfo.md)
- [UserMessage](docs/UserMessage.md)
- [ValidationError](docs/ValidationError.md)
- [ValidationErrorLocInner](docs/ValidationErrorLocInner.md)
- [WandbIntegration](docs/WandbIntegration.md)
- [WandbIntegrationOut](docs/WandbIntegrationOut.md)

To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author
