import FtInstal from "../../model/FtApiActions/FtInstall";
import Ids from "./Ids";

const ftSubmit = (ftInstall: FtInstal) => {
  // ? Idea: https://getbootstrap.com/docs/5.0/forms/validation/
  const dbUsrContainer = document.getElementById(Ids.dbUsr) as HTMLInputElement;
  const dbPasswdContainer = document.getElementById(Ids.dbPasswd) as HTMLInputElement;
  const serverPortContainer = document.getElementById(Ids.serverPort) as HTMLInputElement;

  const dbUsr = dbUsrContainer.value;
  const dbUsrPasswd = dbPasswdContainer.value;
  const serverPort = serverPortContainer.value;

  const resetField = (f: HTMLInputElement) => {
    f.classList.remove("is-invalid");
    f.classList.remove("is-valid");
  };
  [dbUsrContainer, dbPasswdContainer, serverPortContainer].forEach(resetField);
  let isValid: boolean = true;
  if (dbUsr === "") {
    dbUsrContainer.classList.add("is-invalid");
    isValid = false;
  }
  else {
    dbUsrContainer.classList.add("is-valid");
  }
  if (dbUsrPasswd === "") {
    dbPasswdContainer.classList.add("is-invalid");
    isValid = false;
  }
  else {
    dbPasswdContainer.classList.add("is-valid");
  }
  if (serverPort === "") {
    serverPortContainer.classList.add("is-invalid");
    isValid = false;
  }
  else {
    const port = parseInt(serverPort);
    if (isNaN(port) || port < 0 || port > 65535) {
      serverPortContainer.classList.add("is-invalid");
      isValid = false;
    }
    else {
      serverPortContainer.classList.add("is-valid");
    }
  }
  if (!isValid) {
    return;
  }
  ftInstall(dbUsr, dbUsrPasswd, parseInt(serverPort));
};

export default ftSubmit;