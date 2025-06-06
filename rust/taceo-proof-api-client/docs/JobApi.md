# \JobApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_results**](JobApi.md#get_results) | **GET** /api/v1/jobs/job/{id} | get job results
[**schedule_job**](JobApi.md#schedule_job) | **POST** /api/v1/jobs/schedule | create a new job



## get_results

> models::JobResults get_results(id)
get job results

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The uuid of the job | [required] |

### Return type

[**models::JobResults**](JobResults.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_job

> models::ScheduleJobResponse schedule_job(a_blueprint_id, b_job_type, input_party0, input_party1, input_party2, c_code)
create a new job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**a_blueprint_id** | **uuid::Uuid** |  | [required] |
**b_job_type** | [**models::JobType**](JobType.md) |  | [required] |
**input_party0** | **std::path::PathBuf** |  | [required] |
**input_party1** | **std::path::PathBuf** |  | [required] |
**input_party2** | **std::path::PathBuf** |  | [required] |
**c_code** | Option<**String**> |  |  |

### Return type

[**models::ScheduleJobResponse**](ScheduleJobResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

