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
    {mesas.map((mesa, i) => (<div key={i}>
      <br />
      <div className="container">
        <div className="row">
          <div className="col-5">
            <h5>Table {mesa.name}</h5>
          </div>
        </div>
        <div className="row">
          <div className="col">
            {mesa.session? "In progress" : "Available"}
          </div>
          {mesa.session &&
            <>
              <div className="col">{btn("details", () => {onDetails(mesa.session)})}</div>
              <div className="col">{btn("bill", () => {onBill(mesa.session)})}</div>
              <div className="col">{btn("end", () => {endSession(mesa)})}</div>
            </>
          }
          {!mesa.session &&
            <>
              <div className="col"></div>
              <div className="col"></div>
              <div className="col">{btn("new session", () => {newSession(mesa)})}</div>
            </>
          }
        </div>
      </div>
    </div>))}
  </>;
}

const btn = (txt: string, action: () => void) => (
  <button className="btn btn-primary w-100" onClick={action}>{txt}</button>
);

export default Tables;