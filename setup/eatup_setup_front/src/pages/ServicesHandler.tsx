import { useEffect, useState } from "react";

interface Props {
    ftUninstall: () => void;
}

const ServicesHandler = ({ftUninstall}: Props) => {
    const [microservices, setMicroservices] = useState<any[]>([]);

    const updateMicroservices = () => {
        fetch(
          "http://localhost:9000/api/v1/microservices",
          {method: "GET"}
        ).then(async (response) => {
            if (response.status === 200) {
                setMicroservices(await response.json());
                updateMicroservices();
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

    return <>
        <h1>Services</h1>
        <button onClick={ftUninstall}>Uninstall</button>
        <br />
        {/* <div className="mt-5 px-lg-5 container">
        <h1 className="mb-3">Services</h1>
        <Services services={services}/>
    </div> */}
        {microservices.map((service) => {
            return <div key={service.name}>
                <br />
                <h2>{service.name}</h2>
                <ul>
                    <li>State: {service.state}</li>
                    <li>IP: {service.ip}</li>
                    <li>Port: {service.port}</li>
                </ul>
                {/* <ul className="list-group">
      {services.map((service, index) => (
        <li 
          key={index}
          className="list-group-item"
        >
          <div>
            <div className="d-flex justify-content-between">
              <h5 className="mb-1">{service.name}</h5>
              <small>{service.state}</small>
            </div>
          </div>
        </li>
      ))}
    </ul> */}
                <button onClick={() => start(service.name)}>Start</button>
                <button onClick={() => stop(service.name)}>Stop</button>
                <br />
            </div>;
        })}
    </>;
}

export default ServicesHandler;