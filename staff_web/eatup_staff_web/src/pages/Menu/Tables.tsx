import { useEffect, useState } from "react";
import Session from "../../model/api/Session";
import StaffAPI from "../../services/StaffApi";

interface Props {
  onDetails: (session: Session) => void;
  onBill: (session: Session) => void;
}

const Tables = ({onDetails, onBill}: Props) => {
  const [sessions, setSessions] = useState<Session[] | null>(null);

  const updateSessions = () => {
    StaffAPI.getSessions(
      (sessions) => {
        setSessions(Session.fromJsonArray(sessions));
      }
    )
  };

  useEffect(updateSessions, []);

  const newSession = (mesa: any) => {
    StaffAPI.newSession(
      mesa.name,
      (session) => {
        console.log(session);
        localStorage.setItem(session.id, JSON.stringify(session));
        updateSessions();
      }
    )
  };

  const endSession = (mesa: any) => {
    StaffAPI.endSession(
      mesa.session.id,
      (r) => {
        console.log(r);
        updateSessions();
      }
    );
  };

  if (sessions == null) {
    return <p>Loading...</p>;
  }

  let mesasNames = ["10", "11", "12", "13", "14", "15"];
  let mesas: any[] = mesasNames.map((mesaName) => {return {name: mesaName, session: null};});
  for (let i = 0; i < sessions.length; i++) {
    let tableId = sessions[i].table_id;
    if (!sessions[i].in_progress)
      continue;
    if (!/^1\d$/.test(tableId))
      continue;
    let mesaIndex = parseInt(tableId) - 10;
    mesas[mesaIndex].session = sessions[i];
  }

  return <>
    <h1>Tables</h1>
    {mesas.map((mesa, i) => (
      <div key={`${mesa.name}-${i}`}>
        <h5>Mesa {mesa.name}</h5>
        <div style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-between",
        }}>
          <span>{mesa.session? "In progress" : "Available"}</span>
          {mesa.session &&
            <>
              <button onClick={() => {onDetails(mesa.session)}}>details</button>
              <button onClick={() => {onBill(mesa.session)}}>bill</button>
              <button onClick={() => {endSession(mesa)}}>end</button>
            </>
          }
          {!mesa.session &&
            <button onClick={() => {newSession(mesa)}}>new session</button>
          }
        </div>
        <br />
      </div>
    ))}
  </>;
}

export default Tables;