import { useEffect, useState } from "react";
import StaffAPI from "../../services/StaffApi";
import Session from "../../model/api/Session";

interface Props {
  onBill: (selected: Session) => void;
}

const Sessions = ({onBill}: Props) => {
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
    <div className="container text-center">
      {sessions.map((session) => {
        return (
          <div key={session.id} className="row">
            <div className="col-5">{session.id}</div>
            <div className="col">{session.table_id}</div>
            <div className="col">
              {session.in_progress && "In progress"}
            </div>
            <div className="col">
              <button className="btn btn-primary" 
                onClick={() => onBill(session)}
              >Bill</button>
            </div>
          </div>
        );
      })}
    </div>
  </>;
}

export default Sessions;