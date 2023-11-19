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

import { QueryApiError, QueryLoginRequest, QueryUserResponse } from "./data-contracts";
import { ContentType, RequestParams, http } from "./http-client";

export const authApi = {
  /**
   * No description
   *
   * @tags auth
   * @name Login
   * @request POST:/api/v1/auth/login
   * @response `200` `QueryUserResponse` Login user.
   * @response `500` `QueryApiError` Internal server error
   */
  login: (data: QueryLoginRequest, params: RequestParams = {}) =>
    http.request<QueryUserResponse, QueryApiError>({
      path: `/api/v1/auth/login`,
      method: "POST",
      body: data,
      type: ContentType.Json,
      format: "json",
      ...params
    }),

  /**
   * No description
   *
   * @tags auth
   * @name Logout
   * @request GET:/api/v1/auth/logout
   * @response `200` `void` Logout user.
   * @response `500` `QueryApiError` Internal server error
   */
  logout: (params: RequestParams = {}) =>
    http.request<void, QueryApiError>({
      path: `/api/v1/auth/logout`,
      method: "GET",
      ...params
    })
};
