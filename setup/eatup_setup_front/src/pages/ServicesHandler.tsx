import { useEffect, useState } from "react";
import MicroService from "../model/Microservices/MicroService";
import MicroServicesContainer from "../components/servicesHandler/MicroServicesContainer";
import FtUninstall from "../model/FtApiActions/FtUninstall";
import SetupApi from "../services/SetupApi";
import MicroserviceAction from "../model/Microservices/MicroserviceAction";

interface Props {
    ftUninstall: FtUninstall;
}

const ServicesHandler = ({ftUninstall}: Props) => {
    const [microservices, setMicroservices] = useState<MicroService[]>([]);

    const updateMicroservices = () => {
        SetupApi.getMicroservices(
            (microservices: MicroService[]) => {
                setMicroservices(microservices);
            },
            (e: string) => {
                console.error(
                    "Error getting microservices\n",
                    e
                );
            }
        );
    };

    useEffect(updateMicroservices, []);

    const doAction = (action: MicroserviceAction, name: string) => {
        SetupApi.doMicroserviceAction(
            action, name,
            updateMicroservices,
            (e: string) => {
                console.error(
                    "Error doing microservice action\n",
                    e
                );
            }
        );
    }
    
    const start = (name: string) => doAction(MicroserviceAction.Start, name);
    const stop = (name: string) => doAction(MicroserviceAction.Stop, name);

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