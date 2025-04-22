# \NpsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list**](NpsApi.md#list) | **GET** /api/v1/nps/list | get paginated node providers - used for blueprint creators to define clusters
[**register**](NpsApi.md#register) | **POST** /api/v1/nps/register | register a new node provider



## list

> Vec<models::NpsHeader> list(cursor, per_page)
get paginated node providers - used for blueprint creators to define clusters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |

### Return type

[**Vec<models::NpsHeader>**](NpsHeader.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register

> models::RegisterNpsResponse register(register_nps_request)
register a new node provider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_nps_request** | [**RegisterNpsRequest**](RegisterNpsRequest.md) |  | [required] |

### Return type

[**models::RegisterNpsResponse**](RegisterNpsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

