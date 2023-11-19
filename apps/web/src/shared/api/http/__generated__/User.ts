/* eslint-disable */
/* tslint:disable */
/*
 * ---------------------------------------------------------------
 * ## THIS FILE WAS GENERATED VIA SWAGGER-TYPESCRIPT-API        ##
 * ##                                                           ##
 * ## AUTHOR: acacode                                           ##
 * ## SOURCE: https://github.com/acacode/swagger-typescript-api ##
 * ---------------------------------------------------------------
 */

import {
  QueryApiError,
  QueryCreateUserRequest,
  QueryUpdateUserRequest,
  QueryUserResponse,
  QueryUsersResponse
} from "./data-contracts";
import { ContentType, RequestParams, http } from "./http-client";

export const userApi = {
  /**
   * No description
   *
   * @tags user
   * @name CreateUser
   * @request POST:/api/v1/user/create-user
   * @response `200` `QueryUsersResponse` Create new user.
   * @response `500` `QueryApiError` Internal server error
   */
  createUser: (data: QueryCreateUserRequest, params: RequestParams = {}) =>
    http.request<QueryUsersResponse, QueryApiError>({
      path: `/api/v1/user/create-user`,
      method: "POST",
      body: data,
      type: ContentType.Json,
      format: "json",
      ...params
    }),

  /**
   * No description
   *
   * @tags user
   * @name DeleteUser
   * @request DELETE:/api/v1/user/delete-user/{id}
   * @response `200` `void` Update user
   * @response `500` `QueryApiError` Internal server error
   */
  deleteUser: (id: string, params: RequestParams = {}) =>
    http.request<void, QueryApiError>({
      path: `/api/v1/user/delete-user/${id}`,
      method: "DELETE",
      ...params
    }),

  /**
   * No description
   *
   * @tags user
   * @name GetUser
   * @request GET:/api/v1/user/get-user/{id}
   * @response `200` `QueryUserResponse` Get user data.
   * @response `500` `QueryApiError` Internal server error
   */
  getUser: (id: string, params: RequestParams = {}) =>
    http.request<QueryUserResponse, QueryApiError>({
      path: `/api/v1/user/get-user/${id}`,
      method: "GET",
      format: "json",
      ...params
    }),

  /**
   * No description
   *
   * @tags user
   * @name GetUsers
   * @request GET:/api/v1/user/get-users
   * @response `200` `QueryUsersResponse` Get users data.
   * @response `500` `QueryApiError` Internal server error
   */
  getUsers: (params: RequestParams = {}) =>
    http.request<QueryUsersResponse, QueryApiError>({
      path: `/api/v1/user/get-users`,
      method: "GET",
      format: "json",
      ...params
    }),

  /**
   * No description
   *
   * @tags user
   * @name UpdateUser
   * @request PATCH:/api/v1/user/update-user/{id}
   * @response `200` `QueryUsersResponse` Update user
   * @response `500` `QueryApiError` Internal server error
   */
  updateUser: (id: string, data: QueryUpdateUserRequest, params: RequestParams = {}) =>
    http.request<QueryUsersResponse, QueryApiError>({
      path: `/api/v1/user/update-user/${id}`,
      method: "PATCH",
      body: data,
      type: ContentType.Json,
      format: "json",
      ...params
    })
};
