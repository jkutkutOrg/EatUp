import ApiEndpoint from "../model/ApiEndpoint";
import FtCallback from "../model/Futures/FtCallback";
import FtVoid from "../model/Futures/FtVoid";
import API, { ApiMethod } from "./API";

function apiURL(): string {
    const urlParams = new URLSearchParams(window.location.search);
    const ip = urlParams.get("ip") || "localhost";
    const port = urlParams.get("port") || "80";
    return `http://${ip}:${port}`;
}

const toJson = async (response: Response) => await response.json();
const toText = async (response: Response) => await response.text();

class StaffAPI extends API {
    private static readonly url = apiURL();

    private static formatEndpoint(endpoint: ApiEndpoint, options: string[]): string {
        if (options.length === 0)
            return endpoint;
        else
            return `${endpoint}/${options.join("/")}`;
    }

    private static basicGet(
        endpoint: ApiEndpoint,
        options: string[],
        then: FtCallback<any, any>,
        error: FtCallback<string, any>
    ): Promise<Response> {
        return this.get<Response, string>(
            this.url,
            this.formatEndpoint(endpoint, options),
            then, error,
            toJson, toText
        );
    }

    private static basicPost(
        endpoint: ApiEndpoint,
        options: string[],
        body: any | undefined,
        then: FtVoid,
        error: FtCallback<string, any>
    ): Promise<Response> {
        return this.post<string>(
            this.url,
            `${endpoint}/${options.join("/")}`,
            body,
            then,
            error, toText
        );
    }

    private static responsePost(
        endpoint: ApiEndpoint,
        options: string[],
        body: any | undefined,
        then: FtCallback<any, any>,
        error: FtCallback<string, any>
    ): Promise<Response> {
        return this.postResponse(
            this.url,
            `${endpoint}/${options.join("/")}`,
            body,
            then,
            error,
            toJson, toText
        );
    }

    private static responsePatch(
        endpoint: ApiEndpoint | string,
        options: string[],
        body: any | undefined,
        then: FtCallback<any, any>,
        error: FtCallback<string, any>
    ): Promise<Response> {
        return this.patchResponse(
            this.url,
            `${endpoint}/${options.join("/")}`,
            body,
            then,
            error,
            toJson, toText
        );
    }

    public static getSessions(
        then: FtCallback<any, any>,
        error?: FtCallback<string, any>
    ): Promise<Response> {
        return this.basicGet(
            ApiEndpoint.Sessions, [],
            then,
            error || console.error
        );
    }

    public static newSession(
        table_id: string,
        then: FtCallback<any, any>,
        error?: FtCallback<string, any>
    ): Promise<Response> {
        return this.responsePost(
            ApiEndpoint.NewSession, [table_id],
            undefined,
            then,
            error || console.error
        );
    }

    public static endSession(
        session_id: string,
        then: FtCallback<any, any>,
        error?: FtCallback<string, any>
    ): Promise<Response> {
        return this.responsePatch(
            `${ApiEndpoint.EndSessionPre}/${session_id}${ApiEndpoint.EndSessionPost}`,
            [],
            undefined,
            then,
            error || console.error
        );
    }
}

export default StaffAPI;