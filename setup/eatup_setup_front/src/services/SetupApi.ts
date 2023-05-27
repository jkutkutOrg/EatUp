import API from "./API";
import FtCallback from "./FtCallback";

enum Endpoint {
    Status = "/status",
}

class SetupApi extends API {
    private static readonly url = "http://localhost:9000/api/v1";

    private static basicGet(
        endpoint: Endpoint,
        then: FtCallback<any, any>,
        error: FtCallback<string, any>
    ): Promise<Response> {
        return this.get<Response, string>(
            this.url,
            endpoint,
            then,
            error,
            async (response: Response) => await response.json(),
            async (response: Response) => await response.text()
        );
    }

    public static getStatus(
        then: FtCallback<any, any>,
        error: FtCallback<string, any>
    ): Promise<Response> {
        return this.basicGet(
            Endpoint.Status,
            then,
            error
        );
    }
}

export default SetupApi;