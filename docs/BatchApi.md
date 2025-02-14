# \BatchApi

All URIs are relative to *https://api.mistral.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**jobs_api_routes_batch_cancel_batch_job**](BatchApi.md#jobs_api_routes_batch_cancel_batch_job) | **POST** /v1/batch/jobs/{job_id}/cancel | Cancel Batch Job
[**jobs_api_routes_batch_create_batch_job**](BatchApi.md#jobs_api_routes_batch_create_batch_job) | **POST** /v1/batch/jobs | Create Batch Job
[**jobs_api_routes_batch_get_batch_job**](BatchApi.md#jobs_api_routes_batch_get_batch_job) | **GET** /v1/batch/jobs/{job_id} | Get Batch Job
[**jobs_api_routes_batch_get_batch_jobs**](BatchApi.md#jobs_api_routes_batch_get_batch_jobs) | **GET** /v1/batch/jobs | Get Batch Jobs



## jobs_api_routes_batch_cancel_batch_job

> models::BatchJobOut jobs_api_routes_batch_cancel_batch_job(job_id)
Cancel Batch Job

Request the cancellation of a batch job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::BatchJobOut**](BatchJobOut.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_api_routes_batch_create_batch_job

> models::BatchJobOut jobs_api_routes_batch_create_batch_job(batch_job_in)
Create Batch Job

Create a new batch job, it will be queued for processing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_job_in** | [**BatchJobIn**](BatchJobIn.md) |  | [required] |

### Return type

[**models::BatchJobOut**](BatchJobOut.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_api_routes_batch_get_batch_job

> models::BatchJobOut jobs_api_routes_batch_get_batch_job(job_id)
Get Batch Job

Get a batch job details by its UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::BatchJobOut**](BatchJobOut.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_api_routes_batch_get_batch_jobs

> models::BatchJobsOut jobs_api_routes_batch_get_batch_jobs(page, page_size, model, metadata, created_after, created_by_me, status)
Get Batch Jobs

Get a list of batch jobs for your organization and user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 0]
**page_size** | Option<**i32**> |  |  |[default to 100]
**model** | Option<**String**> |  |  |
**metadata** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  |  |
**created_after** | Option<**String**> |  |  |
**created_by_me** | Option<**bool**> |  |  |[default to false]
**status** | Option<[**BatchJobStatus**](.md)> |  |  |

### Return type

[**models::BatchJobsOut**](BatchJobsOut.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

