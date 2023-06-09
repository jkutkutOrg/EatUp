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
};

const getMesas: () => Mesa[] = () => {
  let mesasStr = localStorage.getItem("mesas");
  if (mesasStr == null) {
    mesasStr = JSON.stringify([
      "10", "11", "12", "13", "14", "15"
    ]);
    localStorage.setItem("mesas", mesasStr);
  }
  return JSON.parse(mesasStr).map((mesaName: string) => new Mesa(mesaName));
};

const saveMesas = (mesas: Mesa[]) => {
  localStorage.setItem("mesas", JSON.stringify(mesas.map((mesa) => mesa.getName())));
  let mesasStr = localStorage.getItem("mesas");
}

const Tables = ({onDetails, onBill}: Props) => {
  const [sessions, setSessions] = useState<Session[] | null>(null);
  const [mesitas, setMesitas] = useState<Mesa[]>(getMesas());
  const [showUntracked, setShowUntracked] = useState<boolean>(false);

  const updateSessions = () => {
    StaffAPI.getSessions(
      (sessions) => {
        setSessions(Session.fromJsonArray(sessions));
      }
    )
  }; useEffect(updateSessions, []);

  const newSession = (mesa: Mesa) => {
    StaffAPI.newSession(
      mesa.getName(),
      (session) => {
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

  const removeTable = (mesa: Mesa) => {
    setMesitas(mesitas.filter((m) => m.getName() != mesa.getName()));
  };

  const addTable = (mesa: Mesa) => {
    if (mesitas.find((m) => m.getName() == mesa.getName()) != null) {
      return "Table already exists";
    }
    setMesitas([...mesitas, new Mesa(mesa.getName())]);
    return null;
  };

  // TODO updateSessions when focused

  if (sessions == null) {
    return <p>Loading...</p>;
  }

  saveMesas(mesitas); // TODO only save when changed
  const mesas: Mesa[] = mesitas.map((mesa) => new Mesa(mesa.getName()));
  mesas.sort((a, b) => a.getName().localeCompare(b.getName()));
  const untrackedMesas: Mesa[] = [];
  for (let i = 0; i < sessions.length; i++) {
    if (!sessions[i].in_progress)
      continue;
    let mesa = mesas.find((m) => m.getName() == sessions[i].table_id);
    if (mesa) {
      mesa.setSession(sessions[i]);
    }
    else {
      const newMesa = new Mesa(sessions[i].table_id);
      newMesa.setSession(sessions[i]);
      untrackedMesas.push(newMesa);
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
    <br />
    {untrackedMesas.length > 0 && (<>
      <br />
      <div className="container">
        <div className="row">
          {showUntracked && (<>
            <div className="col-8">
              <h2>Untracked tables</h2>
            </div>
            <div className="col-4">
              <EatupButton type="primary" onClick={() => {
                setShowUntracked(false);
              }}>Hide</EatupButton>
            </div>
          </>) || (<>
            <div className="col">
              <EatupButton type="primary" onClick={() => {
                setShowUntracked(true);
              }}>Show untracked tables</EatupButton>
            </div>
          </>)}
        </div>
      </div>
      {showUntracked && untrackedMesas.map((mesa, i) => (
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
    </>)}
  </>;
};

export default Tables;