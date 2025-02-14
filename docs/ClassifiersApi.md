# \ClassifiersApi

All URIs are relative to *https://api.mistral.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**moderations_chat_v1_chat_moderations_post**](ClassifiersApi.md#moderations_chat_v1_chat_moderations_post) | **POST** /v1/chat/moderations | Moderations Chat
[**moderations_v1_moderations_post**](ClassifiersApi.md#moderations_v1_moderations_post) | **POST** /v1/moderations | Moderations



## moderations_chat_v1_chat_moderations_post

> models::ClassificationResponse moderations_chat_v1_chat_moderations_post(chat_classification_request)
Moderations Chat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_classification_request** | [**ChatClassificationRequest**](ChatClassificationRequest.md) |  | [required] |

### Return type

[**models::ClassificationResponse**](ClassificationResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## moderations_v1_moderations_post

> models::ClassificationResponse moderations_v1_moderations_post(classification_request)
Moderations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**classification_request** | [**ClassificationRequest**](ClassificationRequest.md) |  | [required] |

### Return type

[**models::ClassificationResponse**](ClassificationResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

