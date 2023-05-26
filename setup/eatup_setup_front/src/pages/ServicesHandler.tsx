import { useEffect, useState } from "react";
import MicroService from "../model/MicroService";
import MicroServicesContainer from "../components/servicesHandler/MicroServicesContainer";

interface Props {
    ftUninstall: () => void;
}

const ServicesHandler = ({ftUninstall}: Props) => {
    const [microservices, setMicroservices] = useState<MicroService[]>([]);

    const updateMicroservices = () => {
        fetch(
          "http://localhost:9000/api/v1/microservices",
          {method: "GET"}
        ).then(async (response) => {
            if (response.status === 200) {
                setMicroservices(MicroService.fromJsonArray(await response.json()));
            }
            else {
                console.error(
                    "Error getting microservices\n",
                    await response.json()
                );
            }
        });
    };

    useEffect(updateMicroservices, []);

    const do_action = (action: string, name: string) => {
        console.log("Doing action", action, "on", name);
        fetch(
          `http://localhost:9000/api/v1/microservices/${action}/${name}`,
          {method: "POST"}
        ).then(async (response) => {
            if (response.status === 200) {
                updateMicroservices();
            }
            else {
                console.error(
                    "Error starting microservice\n",
                    await response.text()
                );
            }
        });
    }
    
    const start = (name: string) => do_action("start", name);
    const stop = (name: string) => do_action("stop", name);

    console.log("ServicesHandler", microservices);
    return <>
        <MicroServicesContainer
            microservices={microservices}
            ftStartMicroservice={start}
            ftStopMicroservice={stop}
        />
        <div className="mt-5 px-lg-5 container">
            <div className="d-flex justify-content-center">
                <button className="btn btn-danger" onClick={ftUninstall}>Uninstall</button>
            </div>
        </div>
    </>;
}

export default ServicesHandler;