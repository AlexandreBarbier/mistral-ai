# Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | The ID of the job. | 
**auto_start** | **bool** |  | 
**hyperparameters** | [**models::TrainingParameters**](TrainingParameters.md) |  | 
**model** | [**models::FineTuneableModel**](FineTuneableModel.md) |  | 
**status** | **String** | The current status of the fine-tuning job. | 
**job_type** | **String** | The type of job (`FT` for fine-tuning). | 
**created_at** | **i32** | The UNIX timestamp (in seconds) for when the fine-tuning job was created. | 
**modified_at** | **i32** | The UNIX timestamp (in seconds) for when the fine-tuning job was last modified. | 
**training_files** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | A list containing the IDs of uploaded files that contain training data. | 
**validation_files** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**object** | Option<**String**> |  | [optional][default to JobPeriodMetadata]
**fine_tuned_model** | Option<**String**> |  | [optional]
**suffix** | Option<**String**> |  | [optional]
**integrations** | Option<[**Vec<models::JobOutIntegrationsInner>**](JobOut_integrations_inner.md)> |  | [optional]
**trained_tokens** | Option<**i32**> |  | [optional]
**repositories** | Option<[**Vec<models::JobOutRepositoriesInner>**](JobOut_repositories_inner.md)> |  | [optional][default to []]
**metadata** | Option<[**models::JobMetadataOut**](JobMetadataOut.md)> |  | [optional]
**expected_duration_seconds** | Option<**i32**> |  | [optional]
**cost** | Option<**f64**> |  | [optional]
**cost_currency** | Option<**String**> |  | [optional]
**train_tokens_per_step** | Option<**i32**> |  | [optional]
**train_tokens** | Option<**i32**> |  | [optional]
**data_tokens** | Option<**i32**> |  | [optional]
**estimated_start_time** | Option<**i32**> |  | [optional]
**deprecated** | Option<**bool**> |  | [optional][default to true]
**details** | **String** |  | 
**epochs** | Option<**f64**> |  | [optional]
**training_steps** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


