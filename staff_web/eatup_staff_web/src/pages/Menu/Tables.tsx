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

const getMesas: () => Mesa[] = () => {
  return [
    "10", "11", "12", "13", "14", "15" // TODO Rework
  ].map((mesaName) => new Mesa(mesaName));
}

const Tables = ({onDetails, onBill}: Props) => {
  const [sessions, setSessions] = useState<Session[] | null>(null);
  // const [mesitas, setMesas] = useState<Mesa[] | null>(getMesas());

  const updateSessions = () => {
    StaffAPI.getSessions(
      (sessions) => {
        setSessions(Session.fromJsonArray(sessions));
      }
    )
  };

  const removeTable = (mesa: Mesa) => {
    console.log("Removing table " + mesa.getName());
  };

  const addTable = (mesa: Mesa) => {
    console.log("Adding table " + mesa.getName());
    let valid = true;
    for (let i = 0; i < mesas.length; i++) {
      if (mesas[i].getName() == mesa.getName()) {
        valid = false;
        break;
      }
    }
    if (!valid) {
      return "Table already added";
    }
    // setMesas([...mesas, mesa]);
    return null;
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
        updateSessions();
      }
    );
  };

  if (sessions == null/* || mesitas == null*/) {
    return <p>Loading...</p>;
  }

  let mesitas = getMesas();
  let mesas: Mesa[] = mesitas.slice(); // Copy
  for (let i = 0, j, found; i < sessions.length; i++) {
    let tableId = sessions[i].table_id;
    if (!sessions[i].in_progress)
      continue;
    found = false;
    for (j = 0; j < mesas.length; j++) {
      if (mesas[j].getName() == tableId) {
        mesas[j].setSession(sessions[i]);
        found = true;
        break;
      }
    }
    if (!found) {
      // TODO
    }
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
          removeTable={removeTable}
        />
      </div>
    ))}
    <hr />
    <TableAdder
      onAdd={addTable}
    />
  </>;
};

export default Tables;