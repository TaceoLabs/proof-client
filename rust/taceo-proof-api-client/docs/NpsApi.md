# \NpsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list**](NpsApi.md#list) | **GET** /api/v1/nps/list | get paginated node providers - used for blueprint creators to define clusters
[**register**](NpsApi.md#register) | **POST** /api/v1/nps/register | register a new node provider



## list

> models::PaginationResultNpsHeader list(cursor, per_page)
get paginated node providers - used for blueprint creators to define clusters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |

### Return type

[**models::PaginationResultNpsHeader**](PaginationResult_NpsHeader.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register

> models::RegisterNpsResponse register(cert, enc_key, grpc_url, name, password, verifying_key, version)
register a new node provider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cert** | **String** |  | [required] |
**enc_key** | **String** |  | [required] |
**grpc_url** | **String** |  | [required] |
**name** | **String** |  | [required] |
**password** | **String** |  | [required] |
**verifying_key** | **String** |  | [required] |
**version** | **String** |  | [required] |

### Return type

[**models::RegisterNpsResponse**](RegisterNpsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

