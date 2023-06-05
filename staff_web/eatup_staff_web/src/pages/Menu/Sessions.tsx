import { useEffect, useState } from "react";
import StaffAPI from "../../services/StaffApi";
import Session from "../../model/api/Session";

interface Props {
}

const Sessions = ({}: Props) => {
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
    <h1>Sessions</h1>
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