import { useEffect, useState } from 'react';
import Start from './pages/Start';
import Installation from './pages/Installation';
import ServicesHandler from './pages/ServicesHandler';
import Header from './components/header/Header';
import FtInstall from './model/FtApiActions/FtInstall';
import FtUninstall from './model/FtApiActions/FtUninstall';
import FtCreate from './model/FtApiActions/FtCreate';
import SetupApi from './services/SetupApi';
import ApiEndpoint from './model/ApiEndpoint';

enum Status {
  Connecting = "connecting",
  NotConnected = "not_connected",
  NotCreated = "not_created",
  Created = "created",
  Installed = "installed",
};

const App = () => {
  const [status, setStatus] = useState<string>(Status.Connecting);

  const updateStatus = () => {
    SetupApi.getStatus(
      (rStatus: string) => {
          if (rStatus !== status)
            setStatus(rStatus);
      },
      (e: string) => {
          setStatus(Status.NotConnected);
          console.error(
            "Error getting status\n",
            e
          );
      }
    );
  };

  useEffect(() => {
    window.addEventListener("focus", updateStatus);
    updateStatus();
  }, []);

  const changeAction = (action: ApiEndpoint, body: any | null = null) => {
    SetupApi.changeStatus(
      action,
      body,
      updateStatus,
      (e: string) => {
        console.error(
          `Error doing action ${action}\n`,
          e
        );
      }
    );
  };

  const ftCreate: FtCreate = () => changeAction(ApiEndpoint.Create);

  const ftInstall: FtInstall = (db_usr: string, db_usr_passwd: string, server_port: number) => {
    changeAction(ApiEndpoint.Install, {db_usr, db_usr_passwd, server_port});
  };

  const ftUninstall: FtUninstall = () => changeAction(ApiEndpoint.Uninstall);

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
      body = <Start ftCreate={ftCreate}/>;
      break;
    case Status.Created:
      body = <Installation ftInstall={ftInstall}/>;
      break;
    case Status.Installed:
      body = <ServicesHandler ftUninstall={ftUninstall}/>;
      break;
    default:
      body = <div>Unknown status</div>;
  }

  return <>
    <Header onRefresh={updateStatus}/>
    {body}
  </>;
}

export default App;
