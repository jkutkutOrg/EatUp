import { useEffect, useState } from "react";
import StaffAPI from "../../services/StaffApi";
import Session from "../../model/api/Session";
import EatupButton from "../../components/btn/EatupButton";
import Loading from "../../components/loading/Loading";
import SessionComponent from "../../components/Session/SessionComponent";

interface Props {
  onBill: (selected: Session) => void;
}

const Sessions = ({onBill}: Props) => {
  const [sessions, setSessions] = useState<Session[] | null>(null);
  const [inProgressFilter, setInProgressFilter] = useState<boolean>(false);

  const updateSessions = () => {
    StaffAPI.getSessions(
      (sessions) => {
        sessions.reverse();
        setSessions(Session.fromJsonArray(sessions));
      }
    )
  }; useEffect(updateSessions, []);
  useEffect(() => {
    window.addEventListener("focus", updateSessions);
    return () => {
      window.removeEventListener("focus", updateSessions);
    }
  }, []);

  const toggleFilter = () => {
    setInProgressFilter(!inProgressFilter);
  }

  if (sessions === null) {
    return <Loading />;
  }

  return <>
    <br />
    <div className="container">
      <div className="row">
        <div className="col">
          <h1>Sessions</h1>
        </div>
        <div className="col-4 text-end">
          <EatupButton onClick={toggleFilter}>
            {inProgressFilter ? "Show all" : "Show in progress"}
          </EatupButton>
        </div>
      </div>
    </div>
    <div className="container text-center">
      {sessions.map((session) => {
        return (!inProgressFilter || session.in_progress) && 
          <SessionComponent key={session.id} session={session} onBill={onBill} />
      })}
      <hr />
    </div>
  </>;
}

export default Sessions;