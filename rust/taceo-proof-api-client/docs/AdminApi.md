# \AdminApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_nps_invite_code**](AdminApi.md#create_nps_invite_code) | **POST** /admin/nps/invitation/create | 
[**create_user**](AdminApi.md#create_user) | **POST** /admin/user/create | 
[**create_user_invite_code**](AdminApi.md#create_user_invite_code) | **POST** /admin/user/invitation/create | 
[**paginate_nps_invitations**](AdminApi.md#paginate_nps_invitations) | **GET** /admin/nps/invitation/list | 
[**paginate_user_invitations**](AdminApi.md#paginate_user_invitations) | **GET** /admin/user/invitation/list | 
[**revoke_nps_invitation_code**](AdminApi.md#revoke_nps_invitation_code) | **POST** /admin/nps/invitation/revoke | 
[**revoke_user_invitation_code**](AdminApi.md#revoke_user_invitation_code) | **POST** /admin/user/invitation/revoke | 
[**update_blueprint_access**](AdminApi.md#update_blueprint_access) | **POST** /admin/blueprint/update/access | update the blueprint access
[**update_blueprint_nps**](AdminApi.md#update_blueprint_nps) | **POST** /admin/blueprint/update/nps | update the blueprint nps



## create_nps_invite_code

> String create_nps_invite_code()


### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> create_user(password, username)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_invite_code

> String create_user_invite_code()


### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## paginate_nps_invitations

> models::PaginationResultString paginate_nps_invitations(cursor, per_page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |

### Return type

[**models::PaginationResultString**](PaginationResult_String.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## paginate_user_invitations

> models::PaginationResultString paginate_user_invitations(cursor, per_page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |

### Return type

[**models::PaginationResultString**](PaginationResult_String.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_nps_invitation_code

> revoke_nps_invitation_code(code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_user_invitation_code

> revoke_user_invitation_code(code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_blueprint_access

> update_blueprint_access(access, id)
update the blueprint access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access** | [**models::BlueprintAccess**](BlueprintAccess.md) |  | [required] |
**id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_blueprint_nps

> update_blueprint_nps(id, nps0, nps1, nps2)
update the blueprint nps

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**nps0** | **i32** |  | [required] |
**nps1** | **i32** |  | [required] |
**nps2** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

