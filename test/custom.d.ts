declare function print(...string: string[]): void;

declare class Process {
    public static readonly stdin: {
        [Symbol.asyncIterator](): AsyncIterator<string>
    }
}

declare class Socket {
    [Symbol.asyncIterator](): AsyncIterator<string>
    public write(...message: string[]): void;
}

declare class Network {
    public static bind(port: number): {
        [Symbol.asyncIterator](): AsyncIterator<Socket>
    }
}

declare namespace Http {
    class Server {
        [Symbol.asyncIterator](): AsyncIterator<[Http.Request, Http.Response<any>]>

        get<T>(path: string): {
            [Symbol.asyncIterator](): AsyncIterator<[Http.Request, Http.Response<T>]>
        }

        post<T>(path: string): {
            [Symbol.asyncIterator](): AsyncIterator<[Http.Request, Http.Response<T>]>
        }
    }

    class Request {

    }
    
    class Response<T> {
        status(code: number): Response<T>
        send(body: T): Response<T>
    }
}

declare class Http {
    public static listen(port?: number): Http.Server
}

declare class Authenticator {

}

declare class Crypt {
    public static auth(server: Http.Server): Authenticator
}

declare class Container {
    public static init<T>(path: string): void;
}