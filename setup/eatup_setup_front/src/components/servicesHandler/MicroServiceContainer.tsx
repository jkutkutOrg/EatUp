import MicroService from "../../model/Microservices/MicroService";

interface Props {
    service: MicroService;
    ftStartMicroservice: (name: string) => void;
    ftStopMicroservice: (name: string) => void;
}

const MicroServiceComponent = ({service, ftStartMicroservice, ftStopMicroservice}: Props) => {
    let btnGenerator = (service: MicroService) => {
        let ft = (service.state == "Running") ? ftStopMicroservice : ftStartMicroservice;
        let literal, c;
        if (service.state != "Running") {
            literal = "Start";
            c = "btn-primary";
        }
        else {
            literal = "Stop";
            c = "btn-secondary";
        }
        // ? Idea: disabled
        return <button className={`btn ${c}`} onClick={() => ft(service.name)}>{literal}</button>;
    };

    const port = (service.port)? service.port.split("\n")[0] : "";

    return <tr>
        <th scope="row">{service.name}</th>
        <td>{service.id}</td>
        <td>{service.state}</td>
        <td>{service.ip}</td>
        <td>{port}</td>
        <td>{btnGenerator(service)}</td>
    </tr>;
}

export default MicroServiceComponent;