import type { RunningMode } from "@packages/types";

export type Env = {
    isDevelopment: boolean;
    isProduction: boolean;
    mode: RunningMode;
};

export const env: Env = {
    /**
   * Equals `true` is running in development mode.
   * @readonly
   * @type {boolean}
   */
    isDevelopment: import.meta.env.DEV,
    /**
   * Equals `true` is running in production mode.
   * @readonly
   * @type {boolean}
   */
    isProduction: import.meta.env.PROD,
    mode: import.meta.env.MODE as RunningMode
};
