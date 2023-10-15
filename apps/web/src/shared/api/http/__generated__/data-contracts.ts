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

export type QueryHealthResponse = {
  /** @example "ok" */
  status: string;
  /** @example "1.0.1" */
  version: string;
};
