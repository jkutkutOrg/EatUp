import Mesa from "../../model/App/Mesa";
import Session from "../../model/api/Session";
import TableComponentWrapper from "./TableComponentWrapper";

interface Props {
  mesas: Mesa[];
  onDetails: (session: Session) => void;
  onBill: (session: Session) => void;
  newSession: (mesa: Mesa) => void;
  endSession: (session: Session) => void;
  removeTable: (mesa: Mesa) => void;
};

const TrackedTables = ({
  mesas,
  onDetails,
  onBill,
  newSession,
  endSession,
  removeTable
}: Props) => {
  return <>
    <br />
    <div className="container text-center">
      <h1>Tables</h1>
    </div>
    {mesas.map((mesa, i) => (
      <TableComponentWrapper key={i}
        mesa={mesa}
        onDetails={onDetails}
        onBill={onBill}
        endSession={endSession}
        newSession={newSession}
        removeTable={removeTable}
      />
    ))}
    <hr />
  </>;
};

export default TrackedTables;