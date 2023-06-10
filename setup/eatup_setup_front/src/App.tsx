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
import Status from './model/Status';

const App = () => {
  const [status, setStatus] = useState<Status>(Status.Connecting);

  const updateStatus = () => {
    SetupApi.getStatus(
      (rStatus: string) => {
        let newStatus;
        switch (rStatus) {
          case Status.NotConnected:
            newStatus = Status.NotConnected;
            break;
          case Status.Connecting:
            newStatus = Status.Connecting;
            break;
          case Status.Installed:
            newStatus = Status.Installed;
            break;
          case Status.Created:
            newStatus = Status.Created;
            break;
          case Status.NotCreated:
            newStatus = Status.NotCreated;
          default:
            newStatus = Status.NotConnected;
            break;
        }
        if (newStatus !== status)
          setStatus(newStatus);
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
    <Header status={status} onRefresh={updateStatus}/>
    {body}
  </>;
}

export default App;
