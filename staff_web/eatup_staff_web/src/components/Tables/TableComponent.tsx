import EatupButton from "../../components/btn/EatupButton";
import Mesa from '../../model/App/Mesa';
import Session from "../../model/api/Session";

interface Props {
  mesa: Mesa;

  onDetails: (session: Session) => void;
  onBill: (session: Session) => void;
  endSession: (session: Session) => void;
  newSession: (mesa: Mesa) => void;
  removeTable: (mesa: Mesa) => void;
};

const TableComponent = ({
  mesa,
  onDetails,
  onBill,
  endSession,
  newSession,
  removeTable
}: Props) => {
  const session = mesa.getSession();
  const isSession = session != null;
  return <div className="container">
    <div className="row">
      <div className="col">
        <h5>Table {mesa.getName()}</h5>
      </div>
    </div>
    <div className="row">
      <div className="col-3 align-self-center">
        {isSession? "In progress" : "Available"}
      </div>
      {isSession &&
        <>
          <div className="col">
            <EatupButton onClick={() => {onDetails(session)}}>details</EatupButton>
          </div>
          <div className="col">
            <EatupButton onClick={() => {onBill(session)}}>bill</EatupButton>
          </div>
          <div className="col-4">
            <EatupButton onClick={() => {endSession(session)}}>end</EatupButton>
          </div>
        </> ||
        <>
          <div className="col-7">
            <EatupButton onClick={() => {newSession(mesa)}}>new session</EatupButton>
          </div>
          <div className="col-2">
            <EatupButton type="danger" onClick={() => removeTable(mesa)}>-</EatupButton>
          </div>
        </>
      }
    </div>
  </div>
};

export default TableComponent;