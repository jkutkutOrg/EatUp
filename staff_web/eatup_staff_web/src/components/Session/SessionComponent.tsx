import Session from "../../model/api/Session";
import EatupButton from "../btn/EatupButton";

interface Props {
    session: Session;
    onBill: (session: Session) => void;
};

const SessionComponent = ({session, onBill}: Props) => {
    return (<>
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
            <EatupButton onClick={() => onBill(session)}>
            Bill
            </EatupButton>
        </div>
        </div>
    </>);
};

export default SessionComponent;