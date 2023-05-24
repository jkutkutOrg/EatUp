import MicroService from "../../model/MicroService";

interface Props {
    service: MicroService;
    ftStartMicroservice: (name: string) => void;
    ftStopMicroservice: (name: string) => void;
}

const MicroServiceComponent = ({service, ftStartMicroservice, ftStopMicroservice}: Props) => {
    let btnGenerator = (service: MicroService) => {
        let ft = (service.state == "Running") ? ftStopMicroservice : ftStartMicroservice;
        let literal = (service.state == "Running") ? "Stop" : "Start";
        return <button onClick={() => ft(service.name)}>{literal}</button>;
    };
    // return <div key={service.name}>
    //     <br />
    //     <h2>{service.name}</h2>
    //     <ul>
    //         <li>State: {service.state}</li>
    //         <li>IP: {service.ip}</li>
    //         <li>Port: {service.port}</li>
    //     </ul>
    //     {/* <button onClick={() => ftStartMicroservice(service.name)}>Start</button> */}
    //     {/* <button onClick={() => ftStopMicroservice(service.name)}>Stop</button> */}
    //     {btnGenerator(service)}
    //     <br />
    // </div>;
    return <li className="list-group-item">
        <div>
            <div className="d-flex justify-content-between">
                <h5 className="mb-1">{service.name}</h5>
                <small>{service.id}</small>
                <small>{service.state}</small>
                {service.ip != "" && <small>{service.ip}</small>}
                {/* <small>{service.ip}</small> */}
                {/* <small>{service.port}</small> */}
                {service.port != "" && <small>{service.port}</small>}
                {btnGenerator(service)}
            </div>
        </div>
    </li>;
}

export default MicroServiceComponent;