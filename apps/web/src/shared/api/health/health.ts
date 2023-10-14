import { axiosClient } from "../http-client";

export const healthApi = {
    getHealth: async () => {
        return axiosClient.request({
            method: "GET",
            responseType: "json",
            url: "/health"
        });
    }
};
