# \NpsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list**](NpsApi.md#list) | **GET** /api/v1/nps/list | get paginated node providers



## list

> models::PaginationResultNps list(cursor, per_page)
get paginated node providers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |

### Return type

[**models::PaginationResultNps**](PaginationResult_Nps.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

