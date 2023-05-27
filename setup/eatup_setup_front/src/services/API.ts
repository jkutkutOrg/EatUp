import FtCallback from "./FtCallback";
import FtFuture from "./FtFuture";

const unitFtCallback: FtFuture<any, any> = (data: Promise<any>) => data;

class API {
    public static get<OK, ERROR>(
        url: string,
        endpoint: string,
        ftOk: FtCallback<OK, any>,
        ftErrorCode: FtCallback<ERROR, any>,
        responseOkParser: FtFuture<Response, OK> = unitFtCallback,
        responseErrorCodeParser: FtFuture<Response, ERROR> = unitFtCallback,
        ftError: FtCallback<any, any> | null = null
    ): Promise<Response> {
        ftError = ftError || ftErrorCode;
        return fetch(url + endpoint)
            .then(response => {
                if (response.ok) {
                    return responseOkParser(response).then(ftOk);
                }
                else {
                    return responseErrorCodeParser(response).then(ftErrorCode);
                }
            })
            .catch(ftError);
    }
}

export default API;