# DetailedJobOut

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**auto_start** | **bool** |  | 
**hyperparameters** | [**models::TrainingParameters**](TrainingParameters.md) |  | 
**model** | [**models::FineTuneableModel**](FineTuneableModel.md) |  | 
**status** | **String** |  | 
**job_type** | **String** |  | 
**created_at** | **i32** |  | 
**modified_at** | **i32** |  | 
**training_files** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) |  | 
**validation_files** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**object** | Option<**String**> |  | [optional][default to Job]
**fine_tuned_model** | Option<**String**> |  | [optional]
**suffix** | Option<**String**> |  | [optional]
**integrations** | Option<[**Vec<models::JobOutIntegrationsInner>**](JobOut_integrations_inner.md)> |  | [optional]
**trained_tokens** | Option<**i32**> |  | [optional]
**repositories** | Option<[**Vec<models::JobOutRepositoriesInner>**](JobOut_repositories_inner.md)> |  | [optional][default to []]
**metadata** | Option<[**models::JobMetadataOut**](JobMetadataOut.md)> |  | [optional]
**events** | Option<[**Vec<models::EventOut>**](EventOut.md)> | Event items are created every time the status of a fine-tuning job changes. The timestamped list of all events is accessible here. | [optional][default to []]
**checkpoints** | Option<[**Vec<models::CheckpointOut>**](CheckpointOut.md)> |  | [optional][default to []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


