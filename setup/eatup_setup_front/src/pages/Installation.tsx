interface Props {
    ftInstall: (db_usr: string, db_usr_passwd: string, server_port: number) => void;
}

const Installation = ({ftInstall}: Props) => {
    const submit = () => {
        ftInstall("admin", "admin", 8000); // TODO
    };

    return <>
      <h1>Install menu</h1>
      <p>Project created but more information needed to create the DB and server</p>
      <button onClick={submit}>Install</button>
    </>;
}

export default Installation;