import { useEffect, useState } from "react";
import Session from "../../model/api/Session";
import StaffAPI from "../../services/StaffApi";
import Mesa from "../../model/App/Mesa";
import TableAdder from "../../components/Tables/TableAdder";
import Loading from "../../components/loading/Loading";
import UntrackedTables from "../../components/Tables/UntrackedTables";
import TrackedTables from "../../components/Tables/TrackedTables";

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
}

const Tables = ({onDetails, onBill}: Props) => {
  const [sessions, setSessions] = useState<Session[] | null>(null);
  const [mesitas, setMesitas] = useState<Mesa[]>(getMesas());

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
    const newMesitas = mesitas.filter((m) => m.getName() != mesa.getName());
    saveMesas(newMesitas);
    setMesitas(newMesitas);
  };

  const addTable = (mesa: Mesa) => {
    if (mesitas.find((m) => m.getName() == mesa.getName()) != null) {
      return "Table already exists";
    }
    const newMesitas = [...mesitas, new Mesa(mesa.getName())];
    saveMesas(newMesitas);
    setMesitas(newMesitas);
    return null;
  };
  useEffect(() => {
    window.addEventListener("focus", updateSessions);
    return () => {
      window.removeEventListener("focus", updateSessions);
    }
  }, []);

  if (sessions == null) {
    return <Loading />;
  }

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
    <TrackedTables
      mesas={mesas}
      onDetails={onDetails}
      onBill={onBill}
      newSession={newSession}
      endSession={endSession}
      removeTable={removeTable}
    />
    <TableAdder
      onAdd={addTable}
    />
    <br />
    <UntrackedTables
      untrackedTables={untrackedMesas}
      onDetails={onDetails}
      onBill={onBill}
      endSession={endSession}
    />
  </>;
};

export default Tables;