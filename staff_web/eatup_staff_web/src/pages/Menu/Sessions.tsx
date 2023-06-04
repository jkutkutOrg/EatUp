interface Props {
  
}

const Sessions = ({}: Props) => {
  return <>
    <h1>Sessions</h1>
    {[
      {
        session: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
        table_id: "XX",
      },
      {
        session: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXY",
        table_id: "XX",
      },
      {
        session: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXZ",
        table_id: "XX",
      },
    ].map((session) => {
      return (
        <div key={session.session}>
          <div style={{
            display: "flex",
            flexDirection: "row",
            justifyContent: "space-between",
          }}>
            <span>{session.session}</span>
            <span>{session.table_id}</span>
            <span>bill</span>
          </div>
        </div>
      );
    })}
  </>;
}

export default Sessions;