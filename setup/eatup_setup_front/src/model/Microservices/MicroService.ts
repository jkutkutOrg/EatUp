class MicroService {
    id: string;
    name: string;
    state: string;
    ip: string;
    port: string;

    constructor(
        id: string,
        name: string,
        state: string,
        ip: string,
        port: string
    ) {
        this.id = id;
        this.name = name;
        this.state = state; // ? TODO enum
        this.ip = ip;
        this.port = port;
    }

    static fromJson({id, name, state, ip, port}: any): MicroService {
        return new MicroService(id, name, state, ip, port);
    }

    static fromJsonArray(json: any[]): MicroService[] {
        return json.map(MicroService.fromJson);
    }

    public static arrayFromString(str: string): MicroService[] {
        return MicroService.fromJsonArray(JSON.parse(str));
    }
}

export default MicroService;