import type { RunningMode } from "@internal/types";

export type Env = {
    isDevelopment: boolean;
    isProduction: boolean;
    mode: RunningMode;
    rustApiUrl: string;
};

const isDevelopment = import.meta.env.DEV;
const isLocal = import.meta.env.VITE_LOCAL || ! isDevelopment;

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
    mode: import.meta.env.MODE as RunningMode,
    rustApiUrl: isLocal
        ? "/api"
        : `${import.meta.env.VITE_URL as string}/api`
};
