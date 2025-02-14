# JobOut

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
**object** | Option<**String**> | The object type of the fine-tuning job. | [optional][default to Job]
**fine_tuned_model** | Option<**String**> |  | [optional]
**suffix** | Option<**String**> |  | [optional]
**integrations** | Option<[**Vec<models::JobOutIntegrationsInner>**](JobOut_integrations_inner.md)> |  | [optional]
**trained_tokens** | Option<**i32**> |  | [optional]
**repositories** | Option<[**Vec<models::JobOutRepositoriesInner>**](JobOut_repositories_inner.md)> |  | [optional][default to []]
**metadata** | Option<[**models::JobMetadataOut**](JobMetadataOut.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


