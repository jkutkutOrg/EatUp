import { useEffect, useState } from "react";
import StaffAPI from "../../services/StaffApi";
import Session from "../../model/api/Session";

interface Props {
  onBill: (selected: Session) => void;
}

const Sessions = ({onBill}: Props) => {
  const [sessions, setSessions] = useState<Session[] | null>(null);
  const [inProgressFilter, setInProgressFilter] = useState<boolean>(false);

  useEffect(() => {
    StaffAPI.getSessions(
      (sessions) => {
        setSessions(Session.fromJsonArray(sessions));
      }
    )
  }, []);

  const toggleFilter = () => {
    setInProgressFilter(!inProgressFilter);
  }

  if (sessions === null) {
    return <p>Loading...</p>;
  }

  return <>
    <br />
    <div className="container">
      <div className="row">
        <div className="col">
          <h1>Sessions</h1>
        </div>
        <div className="col text-end">
          <button onClick={toggleFilter}
            className="btn btn-primary btn-dark"
          >
            {inProgressFilter ? "Show all" : "Show in progress"}
          </button>
        </div>
      </div>
    </div>
    <div className="container text-center">
      {sessions.map((session) => {
        if (inProgressFilter && !session.in_progress)
          return;
        return (<div key={session.id}>
          <hr />
          <div className="row">
            <div className="col">
              <h5>{session.id}</h5>
            </div>
          </div>
          <br />
          <div className="row">
            <div className="col">Table {session.table_id}</div>
            <div className="col">
              {session.in_progress && "In progress" || "Finished"}
            </div>
            <div className="col">
              <button className="btn btn-primary w-100"
                onClick={() => onBill(session)}
              >Bill</button>
            </div>
          </div>
        </div>);
      })}
      <hr />
    </div>
  </>;
}

export default Sessions;