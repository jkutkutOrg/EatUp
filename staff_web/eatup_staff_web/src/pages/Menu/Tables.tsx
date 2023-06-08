import { useEffect, useState } from "react";
import Session from "../../model/api/Session";
import StaffAPI from "../../services/StaffApi";
import TableComponent from "../../components/Tables/TableComponent";
import Mesa from "../../model/App/Mesa";
import EatupButton from "../../components/btn/EatupButton";
import TableAdder from "../../components/Tables/TableAdder";

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

  const newSession = (mesa: Mesa) => {
    StaffAPI.newSession(
      mesa.getName(),
      (session) => {
        console.log(session);
        localStorage.setItem(session.id, JSON.stringify(session));
        updateSessions();
      }
    );
  };

  const endSession = (session: Session) => {
    StaffAPI.endSession(
      session.id,
      (r) => {
        console.log(r);
        updateSessions();
      }
    );
  };

  if (sessions == null) {
    return <p>Loading...</p>;
  }

  let mesas: Mesa[] = [
    "10", "11", "12", "13", "14", "15" // TODO Rework
  ].map((mesaName) => new Mesa(mesaName));
  for (let i = 0; i < sessions.length; i++) {
    let tableId = sessions[i].table_id;
    if (!sessions[i].in_progress)
      continue;
    if (!/^1\d$/.test(tableId))
      continue;
    let mesaIndex = parseInt(tableId) - 10;
    mesas[mesaIndex].setSession(sessions[i]);
  }

  return <>
    <br />
    <div className="container text-center">
      <h1>Tables</h1>
    </div>
    {mesas.map((mesa, i) => (
      <div key={i}>
        <hr />
        <TableComponent mesa={mesa}
          onDetails={onDetails}
          onBill={onBill}
          endSession={endSession}
          newSession={newSession}
          removeTable={() => {}}
        />
      </div>
    ))}
    <hr />
    {/* <div className="container">
      <div className="row justify-content-end">
        <div className="col-2">
          <EatupButton type="success" onClick={() => {}}>+</EatupButton>
        </div>
      </div>
    </div> */}
    <TableAdder
      onAdd={(mesaName: Mesa) => {
        console.log("Adding table " + mesaName)
        return null;
      }}
    />
  </>;
};

export default Tables;