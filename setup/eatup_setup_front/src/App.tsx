import { useEffect, useState } from 'react';
import Start from './pages/Start';
import Installation from './pages/Installation';
import ServicesHandler from './pages/ServicesHandler';
import Header from './components/header/Header';

enum Status {
  Connecting = "connecting",
  NotConnected = "not_connected",
  NotCreated = "not_created",
  Created = "created",
  Installed = "installed",
};

// TODO create logic to generalize the API communication

const App = () => {
  const [status, setStatus] = useState<string>(Status.Connecting);

  useEffect(() => {
    window.addEventListener("focus", updateStatus);
    updateStatus();
  }, []);

  const statusAction = (action: string, body: any | null = null) => {
    let init: any = {method: "POST"};
    if (body !== null) {
      init.headers = {
        "Content-Type": "application/json"
      };
      init.body = JSON.stringify(body);
    }
    fetch(
      `http://localhost:9000/api/v1/${action}`,
      init
    ).then(async (response) => {
      if (response.status === 200) {
        updateStatus();
      }
      else {
        console.error(
          "Error starting microservice\n",
          await response.text()
        );
      }
    }).catch((error) => {
      setStatus(Status.NotConnected);
      console.error(
        "Error starting microservice\n",
        error
      );
    });
  };

  const updateStatus = () => {
    fetch(
      "http://localhost:9000/api/v1/status",
      {method: "GET"}
    ).then((response) => {
      if (response.status === 200) {
        return response.json();
      }
      // TODO handle error
    }).then((data) => {
      if (data != status)
        setStatus(data);
    }).catch((error) => {
      setStatus(Status.NotConnected);
    });
  };

  let body;
  switch (status) {
    case Status.Connecting:
      body = <div className="d-flex justify-content-center align-items-center" style={{height: "100vh"}}>
        <div className="spinner-border" role="status">
          <span className="sr-only">Loading...</span>
        </div>
      </div>;
      break;
    case Status.NotConnected:
      body = <div>Not connected. Is the backend running?</div>;
      break;
    case Status.NotCreated:
      body = <Start ftCreate={() => statusAction("create")} />;
      break;
    case Status.Created:
      body = <Installation ftInstall={(db_usr: string, db_usr_passwd: string, server_port: number) => {
        statusAction("install", {db_usr, db_usr_passwd, server_port});
      }} />;
      break;
    case Status.Installed:
      body = <ServicesHandler
        ftUninstall={() => statusAction("uninstall")}
      />;
      break;
    default:
      body = <div>Unknown status</div>;
  }

  return <>
    <Header onRefresh={updateStatus} />
    {body}
  </>
}

export default App;
