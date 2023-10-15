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

import { QueryApiError, QueryHealthResponse } from "./data-contracts";
import { RequestParams, http } from "./http-client";

export const healthApi = {
  /**
   * No description
   *
   * @tags health
   * @name GetHealth
   * @request GET:/api/health
   * @response `200` `QueryHealthResponse` Check api status.
   * @response `500` `QueryApiError` Internal server error
   */
  getHealth: (params: RequestParams = {}) =>
    http.request<QueryHealthResponse, QueryApiError>({
      path: `/api/health`,
      method: "GET",
      format: "json",
      ...params
    })
};
