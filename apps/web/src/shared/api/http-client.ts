import axios from "axios";

import { env } from "./../utils";

export const axiosClient = axios.create({
    baseURL: env.rustApiUrl,
    headers: {
        "Content-Type": "application/json"
    },
    withCredentials: true
});
