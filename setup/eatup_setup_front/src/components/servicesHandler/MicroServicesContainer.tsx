import MicroService from "../../model/MicroService";
import MicroServiceComponent from "./MicroServiceContainer";

interface Props {
    microservices: MicroService[];
    ftStartMicroservice: (name: string) => void;
    ftStopMicroservice: (name: string) => void;
}

const MicroServicesContainer = ({microservices, ftStartMicroservice, ftStopMicroservice}: Props) => {
    let btnGenerator = (service: MicroService, action: string) => {
        let ft = (service.state == "Running") ? ftStopMicroservice : ftStartMicroservice;
        let literal = (service.state == "Running") ? "Stop" : "Start";
        return <button onClick={() => ft(service.name)}>{literal}</button>;
    };
    return <div className="mt-5 px-lg-5 container">
        <h1 className="mb-3">Services!</h1>
        <ul className="list-group">
            {microservices.map((service, index) => (
                <MicroServiceComponent
                    key={index}
                    service={service}
                    ftStartMicroservice={ftStartMicroservice}
                    ftStopMicroservice={ftStopMicroservice}
                />
            ))}
        </ul>
        {/* {microservices.map((service) => 
            <MicroServiceComponent
                service={service}
                ftStartMicroservice={ftStartMicroservice}
                ftStopMicroservice={ftStopMicroservice}
            />
        )} */}
    </div>;
}

export default MicroServicesContainer;