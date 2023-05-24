import { useEffect, useState } from 'react';
import Header from './components/header/Header';
import ServicesHandler from './components/serviceHandler/ServiceHandler';
// import useWebsocket from './model/useWebsocket'; // TODO remove

enum Status {
  NotCreated = "not_created",
  Created = "created",
  Installed = "installed",
};

const App = () => {
  const [status, setStatus] = useState<string>("Not connected");

  const refresh = () => {
    fetch(
      "http://localhost:9000/api/v1/status",
      {method: "GET"}
    ).then((response) => {
      if (response.status === 200) {
        return response.json();
      }
      // setStatus("Error"); // TODO
    }).then((data) => {
      if (data != status)
        setStatus(data);
    });
  };

  useEffect(() => {
    refresh();
  }, []);

  if (status === Status.NotCreated) {
    return <>
      <h1>{status}</h1>
      <button onClick={() => {
        fetch(
          "http://localhost:9000/api/v1/create",
          {method: "POST"}
        ).then((response) => {
          if (response.status === 200)
            refresh();
          else {
            let l = async (response: Response) => {
              console.error(
                "Error installing setup\n",
                await response.json()
              );
            };
            l(response);
          }
        });
      }}>Create</button>
    </>;
  }
  else if (status === Status.Created) {
    return <>
      <h1>{status}</h1>
      <button onClick={() => {
        fetch(
          "http://localhost:9000/api/v1/install",
          {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
              "Access-Control-Request-Method": "POST",
              "Access-Control-Request-Headers": "origin, x-requested-with",
              "Origin": "http://localhost:5173"
            },
            body: JSON.stringify({
              db_usr: "admin",
              db_usr_passwd: "admin",
              server_port: 8000
            })
          }
        ).then((response) => {
          if (response.status === 200) {
            refresh();
          }
          else {
            let l = async (response: Response) => {
              console.error(
                "Error installing setup\n",
                await response.text()
              );
            };
            l(response);
          }
        });
      }}>Install</button>
    </>;
  }

  let [microservices, setMicroservices] = useState<any[]>([]);

  const updateMicroservices = () => {
    fetch(
      "http://localhost:9000/api/v1/microservices",
      {method: "GET"}
    ).then(async (response) => {
      if (response.status === 200) {
        setMicroservices(await response.json());
      }
      else {
        let l = async (response: Response) => {
          console.error(
            "Error getting microservices\n",
            await response.json()
          );
        };
        l(response);
      }
    });
  };

  useEffect(() => {
    updateMicroservices();
  }, []);

  const do_action = (action: string, name: string) => {
    console.log("Doing action", action, "on", name);
    fetch(
      `http://localhost:9000/api/v1/microservices/${action}/${name}`,
      {method: "POST"}
    ).then((response) => {
      if (response.status === 200) {
        updateMicroservices();
      }
      else {
        let l = async (response: Response) => {
          console.error(
            "Error starting microservice\n",
            await response.json()
          );
        };
        l(response);
      }
    });
  }

  const start = (name: string) => do_action("start", name);
  const stop = (name: string) => do_action("stop", name);

  return <>
    {/* <Header
      onRefresh={refresh}
    />
    <ServicesHandler
      services={socket.message}
    /> */}
    <h1>{status}</h1>
    {microservices.map((service) => {
      return <>
        <br />
        <h2>{service.name}</h2>
        <p>{service.id}</p>
        <p>{service.state}</p>
        <br />
        <button onClick={() => start(service.name)}>Start</button>
        <button onClick={() => stop(service.name)}>Stop</button>
      </>;
    })}
  </>
}

export default App;
