# \ModelsApi

All URIs are relative to *https://api.mistral.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_model_v1_models_model_id_delete**](ModelsApi.md#delete_model_v1_models_model_id_delete) | **DELETE** /v1/models/{model_id} | Delete Model
[**jobs_api_routes_fine_tuning_archive_fine_tuned_model**](ModelsApi.md#jobs_api_routes_fine_tuning_archive_fine_tuned_model) | **POST** /v1/fine_tuning/models/{model_id}/archive | Archive Fine Tuned Model
[**jobs_api_routes_fine_tuning_unarchive_fine_tuned_model**](ModelsApi.md#jobs_api_routes_fine_tuning_unarchive_fine_tuned_model) | **DELETE** /v1/fine_tuning/models/{model_id}/archive | Unarchive Fine Tuned Model
[**jobs_api_routes_fine_tuning_update_fine_tuned_model**](ModelsApi.md#jobs_api_routes_fine_tuning_update_fine_tuned_model) | **PATCH** /v1/fine_tuning/models/{model_id} | Update Fine Tuned Model
[**list_models_v1_models_get**](ModelsApi.md#list_models_v1_models_get) | **GET** /v1/models | List Models
[**retrieve_model_v1_models_model_id_get**](ModelsApi.md#retrieve_model_v1_models_model_id_get) | **GET** /v1/models/{model_id} | Retrieve Model



## delete_model_v1_models_model_id_delete

> models::DeleteModelOut delete_model_v1_models_model_id_delete(model_id)
Delete Model

Delete a fine-tuned model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_id** | **String** | The ID of the model to delete. | [required] |

### Return type

[**models::DeleteModelOut**](DeleteModelOut.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_api_routes_fine_tuning_archive_fine_tuned_model

> models::ArchiveFtModelOut jobs_api_routes_fine_tuning_archive_fine_tuned_model(model_id)
Archive Fine Tuned Model

Archive a fine-tuned model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_id** | **String** | The ID of the model to archive. | [required] |

### Return type

[**models::ArchiveFtModelOut**](ArchiveFTModelOut.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_api_routes_fine_tuning_unarchive_fine_tuned_model

> models::UnarchiveFtModelOut jobs_api_routes_fine_tuning_unarchive_fine_tuned_model(model_id)
Unarchive Fine Tuned Model

Un-archive a fine-tuned model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_id** | **String** | The ID of the model to unarchive. | [required] |

### Return type

[**models::UnarchiveFtModelOut**](UnarchiveFTModelOut.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_api_routes_fine_tuning_update_fine_tuned_model

> models::FtModelOut jobs_api_routes_fine_tuning_update_fine_tuned_model(model_id, update_ft_model_in)
Update Fine Tuned Model

Update a model name or description.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_id** | **String** | The ID of the model to update. | [required] |
**update_ft_model_in** | [**UpdateFtModelIn**](UpdateFtModelIn.md) |  | [required] |

### Return type

[**models::FtModelOut**](FTModelOut.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_models_v1_models_get

> models::ModelList list_models_v1_models_get()
List Models

List all models available to the user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ModelList**](ModelList.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_model_v1_models_model_id_get

> models::ResponseRetrieveModelV1ModelsModelIdGet retrieve_model_v1_models_model_id_get(model_id)
Retrieve Model

Retrieve a model information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_id** | **String** | The ID of the model to retrieve. | [required] |

### Return type

[**models::ResponseRetrieveModelV1ModelsModelIdGet**](Response_Retrieve_Model_V1_Models__Model_Id__Get.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

