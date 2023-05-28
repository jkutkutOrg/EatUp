import FtCallback from "../model/Futures/FtCallback";
import FtFuture from "../model/Futures/FtFuture";
import FtVoid from "../model/Futures/FtVoid";

const unitFtCallback: FtFuture<any, any> = (data: Promise<any>) => data;

enum ApiMethod {
    Get = "GET",
    Post = "POST",
};

const ApiHeaders = {
    json: {
        "Content-Type": "application/json"
    }
};

class API {
    private static callResponse<OK, ERROR>(
        url: string,
        endpoint: string,
        method: ApiMethod,
        body: any | undefined,
        ftOk: FtCallback<OK, any>,
        ftErrorCode: FtCallback<ERROR, any>,
        responseOkParser: FtFuture<Response, OK>,
        responseErrorCodeParser: FtFuture<Response, ERROR>,
        ftError: FtCallback<any, any>
    ): Promise<Response> {
        let headers: any | undefined = undefined;
        if (body !== undefined) {
            headers = ApiHeaders.json;
            body = JSON.stringify(body);
        }
        return fetch(
            url + endpoint,
            {
                method: method,
                headers: headers,
                body: body
            }
        ).then(response => {
            if (response.ok)
                return responseOkParser(response).then(ftOk);
            else
                return responseErrorCodeParser(response).then(ftErrorCode);
        }).catch(ftError);
    }

    private static callNoResponse<ERROR>(
        url: string,
        endpoint: string,
        method: ApiMethod,
        body: any | undefined,
        ftOk: FtVoid,
        ftErrorCode: FtCallback<ERROR, any>,
        responseErrorCodeParser: FtFuture<Response, ERROR>,
        ftError: FtCallback<any, any>
    ): Promise<Response> {
        let headers: any | undefined = undefined;
        if (body !== undefined) {
            headers = ApiHeaders.json;
            body = JSON.stringify(body);
        }
        return fetch(
            url + endpoint,
            {
                method: method,
                headers: headers,
                body: body
            }
        ).then(response => {
            if (response.ok)
                return ftOk();
            else
                return responseErrorCodeParser(response).then(ftErrorCode);
        }).catch(ftError);
    }


    public static get<OK, ERROR>(
        url: string,
        endpoint: string,
        ftOk: FtCallback<OK, any>,
        ftErrorCode: FtCallback<ERROR, any>,
        responseOkParser: FtFuture<Response, OK> = unitFtCallback,
        responseErrorCodeParser: FtFuture<Response, ERROR> = unitFtCallback,
        ftError: FtCallback<any, any> | null = null
    ): Promise<Response> {
        return this.callResponse(
            url, endpoint,
            ApiMethod.Get, undefined,
            ftOk, ftErrorCode,
            responseOkParser, responseErrorCodeParser,
            ftError || ftErrorCode
        );
    }

    public static post<ERROR>(
        url: string,
        endpoint: string,
        body: any | undefined,
        ftOk: FtVoid,
        ftErrorCode: FtCallback<ERROR, any>,
        responseErrorCodeParser: FtFuture<Response, ERROR> = unitFtCallback,
        ftError: FtCallback<any, any> | null = null
    ): Promise<Response> {
        return this.callNoResponse(
            url, endpoint,
            ApiMethod.Post, body,
            ftOk, ftErrorCode,
            responseErrorCodeParser,
            ftError || ftErrorCode
        );
    }
}

export default API;