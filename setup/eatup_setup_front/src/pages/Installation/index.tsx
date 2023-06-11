import NumberInput from "../../components/input/NumberInput";
import PasswordInput from "../../components/input/Password";
import TextInput from "../../components/input/TextInput";
import FtInstall from "../../model/FtApiActions/FtInstall";
import Ids from "./Ids";
import ftSubmit from "./ftSubmit";

interface Props {
  ftInstall: FtInstall;
}

const Installation = ({ftInstall}: Props) => {
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
          <span className="input-group-text">{window.location.hostname}</span>
          <span className="input-group-text">:</span>
          <NumberInput id={"serverPort"} placeholder="Server public port"/>
        </div>
        <br />
        <div className="d-grid gap-2">
          <button className="btn btn-primary" onClick={() => {ftSubmit(ftInstall)}}>Install</button>
        </div>
      </div>
    </div>
  </div>);
}

export default Installation;