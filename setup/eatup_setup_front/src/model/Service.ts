class Service {
    name: String;
    state: String;
    requires: String[];

    constructor(
        name: String,
        state: String,
        requires: String[]
    ) {
        this.name = name;
        this.state = state;
        this.requires = requires;
    }

    static fromJson({name, state, requires}: any): Service {
        return new Service(name, state, requires);
    }

    static fromJsonArray(json: any[]): Service[] {
        return json.map(Service.fromJson);
    }

    public static arrayFromString(str: string): Service[] {
        return Service.fromJsonArray(JSON.parse(str));
    }
}

export default Service;