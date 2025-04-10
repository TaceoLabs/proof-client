# \JobApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_input**](JobApi.md#add_input) | **PUT** /api/v1/jobs/{id}/input | add input data to a job
[**issue_cosnark_code**](JobApi.md#issue_cosnark_code) | **GET** /api/v1/jobs/code | create a new job
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


## issue_cosnark_code

> String issue_cosnark_code(issue_co_snark_code_request)
create a new job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_co_snark_code_request** | [**IssueCoSnarkCodeRequest**](IssueCoSnarkCodeRequest.md) |  | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json

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

