# \FineTuningApi

All URIs are relative to *https://api.mistral.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**jobs_api_routes_fine_tuning_cancel_fine_tuning_job**](FineTuningApi.md#jobs_api_routes_fine_tuning_cancel_fine_tuning_job) | **POST** /v1/fine_tuning/jobs/{job_id}/cancel | Cancel Fine Tuning Job
[**jobs_api_routes_fine_tuning_create_fine_tuning_job**](FineTuningApi.md#jobs_api_routes_fine_tuning_create_fine_tuning_job) | **POST** /v1/fine_tuning/jobs | Create Fine Tuning Job
[**jobs_api_routes_fine_tuning_get_fine_tuning_job**](FineTuningApi.md#jobs_api_routes_fine_tuning_get_fine_tuning_job) | **GET** /v1/fine_tuning/jobs/{job_id} | Get Fine Tuning Job
[**jobs_api_routes_fine_tuning_get_fine_tuning_jobs**](FineTuningApi.md#jobs_api_routes_fine_tuning_get_fine_tuning_jobs) | **GET** /v1/fine_tuning/jobs | Get Fine Tuning Jobs
[**jobs_api_routes_fine_tuning_start_fine_tuning_job**](FineTuningApi.md#jobs_api_routes_fine_tuning_start_fine_tuning_job) | **POST** /v1/fine_tuning/jobs/{job_id}/start | Start Fine Tuning Job



## jobs_api_routes_fine_tuning_cancel_fine_tuning_job

> models::DetailedJobOut jobs_api_routes_fine_tuning_cancel_fine_tuning_job(job_id)
Cancel Fine Tuning Job

Request the cancellation of a fine tuning job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | The ID of the job to cancel. | [required] |

### Return type

[**models::DetailedJobOut**](DetailedJobOut.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_api_routes_fine_tuning_create_fine_tuning_job

> models::Response jobs_api_routes_fine_tuning_create_fine_tuning_job(job_in, dry_run)
Create Fine Tuning Job

Create a new fine-tuning job, it will be queued for processing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_in** | [**JobIn**](JobIn.md) |  | [required] |
**dry_run** | Option<**bool**> | * If `true` the job is not spawned, instead the query returns a handful of useful metadata   for the user to perform sanity checks (see `LegacyJobMetadataOut` response). * Otherwise, the job is started and the query returns the job ID along with some of the   input parameters (see `JobOut` response).  |  |

### Return type

[**models::Response**](Response.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_api_routes_fine_tuning_get_fine_tuning_job

> models::DetailedJobOut jobs_api_routes_fine_tuning_get_fine_tuning_job(job_id)
Get Fine Tuning Job

Get a fine-tuned job details by its UUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** | The ID of the job to analyse. | [required] |

### Return type

[**models::DetailedJobOut**](DetailedJobOut.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_api_routes_fine_tuning_get_fine_tuning_jobs

> models::JobsOut jobs_api_routes_fine_tuning_get_fine_tuning_jobs(page, page_size, model, created_after, created_by_me, status, wandb_project, wandb_name, suffix)
Get Fine Tuning Jobs

Get a list of fine-tuning jobs for your organization and user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page number of the results to be returned. |  |[default to 0]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]
**model** | Option<**String**> | The model name used for fine-tuning to filter on. When set, the other results are not displayed. |  |
**created_after** | Option<**String**> | The date/time to filter on. When set, the results for previous creation times are not displayed. |  |
**created_by_me** | Option<**bool**> | When set, only return results for jobs created by the API caller. Other results are not displayed. |  |[default to false]
**status** | Option<**String**> | The current job state to filter on. When set, the other results are not displayed. |  |
**wandb_project** | Option<**String**> | The Weights and Biases project to filter on. When set, the other results are not displayed. |  |
**wandb_name** | Option<**String**> | The Weight and Biases run name to filter on. When set, the other results are not displayed. |  |
**suffix** | Option<**String**> | The model suffix to filter on. When set, the other results are not displayed. |  |

### Return type

[**models::JobsOut**](JobsOut.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_api_routes_fine_tuning_start_fine_tuning_job

> models::DetailedJobOut jobs_api_routes_fine_tuning_start_fine_tuning_job(job_id)
Start Fine Tuning Job

Request the start of a validated fine tuning job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DetailedJobOut**](DetailedJobOut.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

