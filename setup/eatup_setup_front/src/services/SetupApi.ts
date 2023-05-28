import API from "./API";
import FtCallback from "../model/Futures/FtCallback";
import FtVoid from "../model/Futures/FtVoid";
import MicroService from "../model/Microservices/MicroService";
import MicroserviceAction from "../model/Microservices/MicroserviceAction";
import ApiEndpoint from "../model/ApiEndpoint";

const toJson = async (response: Response) => await response.json();
const toText = async (response: Response) => await response.text();

class SetupApi extends API {
    private static readonly url = "http://localhost:9000/api/v1";

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

    public static getStatus(
        then: FtCallback<any, any>,
        error: FtCallback<string, any>
    ): Promise<Response> {
        return this.basicGet(
            ApiEndpoint.Status, [],
            then,
            error
        );
    }

    public static getMicroservices(
        then: FtCallback<MicroService[], any>,
        error: FtCallback<string, any>
    ): Promise<Response> {
        return this.basicGet(
            ApiEndpoint.Microservices, [],
            (data: any) => then(MicroService.fromJsonArray(data)),
            error
        );
    }

    public static changeStatus(
        endpoint: ApiEndpoint,
        body: any | undefined,
        then: FtVoid,
        error: FtCallback<string, any>
    ): Promise<Response> {
        switch (endpoint) {
            case ApiEndpoint.Create:
            case ApiEndpoint.Install:
            case ApiEndpoint.Uninstall:
                break;
            default:
                throw new Error("Invalid endpoint at SetupApi.changeStatus");
        }
        return this.basicPost(
            endpoint, [],
            body,
            then,
            error
        );
    }

    public static doMicroserviceAction(
        action: MicroserviceAction,
        microservice: string,
        then: FtVoid,
        error: FtCallback<string, any>
    ): Promise<Response> {
        return this.basicPost(
            ApiEndpoint.Microservices, [action.toLowerCase(), microservice],
            undefined,
            then,
            error
        );
    }
}

export default SetupApi;
export { ApiEndpoint };