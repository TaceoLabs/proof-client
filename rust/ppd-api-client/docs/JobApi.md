# \JobApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_input**](JobApi.md#add_input) | **PUT** /api/v1/jobs/{id}/input | add input data to a job
[**get_result**](JobApi.md#get_result) | **GET** /api/v1/jobs/job/{id} | get job result
[**schedule_job**](JobApi.md#schedule_job) | **POST** /api/v1/jobs/schedule | create a new job



## add_input

> add_input(id, input_party0, input_party1, input_party2)
add input data to a job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The uuid of the job | [required] |
**input_party0** | **std::path::PathBuf** |  | [required] |
**input_party1** | **std::path::PathBuf** |  | [required] |
**input_party2** | **std::path::PathBuf** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_result

> models::JobResult get_result(id)
get job result

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The uuid of the job | [required] |

### Return type

[**models::JobResult**](JobResult.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_job

> models::ScheduleJobResponse schedule_job(schedule_job_request)
create a new job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_job_request** | [**ScheduleJobRequest**](ScheduleJobRequest.md) |  | [required] |

### Return type

[**models::ScheduleJobResponse**](ScheduleJobResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

