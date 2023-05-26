import Input from "../components/input/Input";
import NumberInput from "../components/input/NumberInput";
import PasswordInput from "../components/input/Password";
import TextInput from "../components/input/TextInput";

interface Props {
    ftInstall: (dbUsr: string, dbUsrPasswd: string, serverPort: number) => void;
}

enum Ids {
    dbUsr = "dbUsr",
    dbPasswd = "dbPasswd",
    serverPort = "serverPort"
}

const Installation = ({ftInstall}: Props) => {
    const submit = () => {
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

    let ip = window.location.hostname;

    return (
    <div className="row text-center">
        <div className="card-body" style={{width: "auto"}}>
            <h2 className="card-title">Installation:</h2>
            <br />
            <div className="container align-items-center" style={{width: "60vw"}}>
                <p>Database configuration</p>
                <div className="input-group mb-3">
                    <span className="input-group-text">Database username:</span>
                    <TextInput id={Ids.dbUsr} placeholder="Example: admin"/>
                </div>
                <div className="input-group mb-3">
                    <span className="input-group-text">Database password:</span>
                    <PasswordInput id={Ids.dbPasswd} placeholder="Example: admin"/>
                </div>
                <p>Server configuration</p>
                <div className="input-group mb-3">
                    <span className="input-group-text">{ip}</span>
                    <span className="input-group-text">:</span>
                    <NumberInput id={"serverPort"} placeholder="Server public port"/>
                </div>
                <br />
                <div className="d-grid gap-2">
                    <button className="btn btn-primary" onClick={submit}>Install</button>
                </div>
            </div>
        </div>
    </div>);
}

export default Installation;