import { useEffect, useState } from "react";
import StaffAPI from "../../services/StaffApi";
import Session from "../../model/api/Session";

interface Props {
  onClose: () => void;
}

const Sessions = ({onClose}: Props) => {
  const [sessions, setSessions] = useState<Session[] | null>(null);

  useEffect(() => {
    StaffAPI.getSessions(
      (sessions) => {
        setSessions(Session.fromJsonArray(sessions));
      }
    )
  }, []);

  if (sessions === null) {
    return <p>Loading...</p>;
  }

  return <>
    <div style={{
      display: "flex",
      flexDirection: "row",
      justifyContent: "space-between",
    }}>
      <h1>Sessions</h1>
      <button onClick={onClose}>Close</button>
    </div>
    {sessions.map((session) => {
      return (
        <div key={session.id}>
          <div style={{
            display: "flex",
            flexDirection: "row",
            justifyContent: "space-between",
          }}>
            <span>{session.id}</span>
            <span>{session.table_id}</span>
            <span>{session.in_progress && "In progress"}</span>
            <span>bill</span> {/* TODO */}
          </div>
        </div>
      );
    })}
  </>;
}

export default Sessions;