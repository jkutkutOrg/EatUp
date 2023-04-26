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

    public static fromJson({name, state, requires}: any): Service {
        return new Service(name, state, requires);
    }

    public static fromJsonArray(json: any[]): Service[] {
        return json.map(Service.fromJson);
    }
}

export default Service;