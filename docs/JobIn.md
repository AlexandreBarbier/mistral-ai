# JobIn

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**model** | [**models::FineTuneableModel**](FineTuneableModel.md) |  | 
**training_files** | Option<[**Vec<models::TrainingFile>**](TrainingFile.md)> |  | [optional][default to []]
**validation_files** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**hyperparameters** | [**models::TrainingParametersIn**](TrainingParametersIn.md) |  | 
**suffix** | Option<**String**> |  | [optional]
**integrations** | Option<[**Vec<models::JobInIntegrationsInner>**](JobIn_integrations_inner.md)> |  | [optional]
**repositories** | Option<[**Vec<models::JobInRepositoriesInner>**](JobIn_repositories_inner.md)> |  | [optional][default to []]
**auto_start** | Option<**bool**> | This field will be required in a future release. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


