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

export type QueryApiError =
  | {
      BadRequest: string;
    }
  | {
      BlockingError: string;
    }
  | {
      CacheError: string;
    }
  | {
      CannotDecodeJwtToken: string;
    }
  | {
      CannotEncodeJwtToken: string;
    }
  | {
      InternalServerError: string;
    }
  | {
      NotFound: string;
    }
  | {
      ParseError: string;
    }
  | {
      PoolError: string;
    }
  | {
      ValidationError: string[];
    }
  | {
      Unauthorized: string;
    };

export type QueryCreateUserRequest = {
  /** @example "test@email.com" */
  email: string;
  /** @example "Name" */
  first_name: string;
  /** @example "Last Name" */
  last_name: string;
  /** @example "xxxxxxx" */
  password: string;
};

export type QueryHealthResponse = {
  /** @example "ok" */
  status: string;
  /** @example "1.0.1" */
  version: string;
};

export type QueryLoginRequest = {
  /** @example "test@email.com" */
  email: string;
  /** @example "xxxxxxxx" */
  password: string;
};

export type QueryUpdateUserRequest = {
  /** @example "test@email.com" */
  email: string;
  /** @example "Name" */
  first_name: string;
  /** @example "Last Name" */
  last_name: string;
};

export type QueryUserResponse = {
  /** @example "test@test.com" */
  email: string;
  /** @example "Name" */
  first_name: string;
  /** @example "550e8400-e29b-41d4-a716-446655440000" */
  id: string;
  /** @example "Last Name" */
  last_name: string;
  /** @example "user" */
  role: string;
};

export type QueryUsersResponse = QueryUserResponse[];
