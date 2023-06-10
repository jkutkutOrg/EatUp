import Mesa from "../../model/App/Mesa";
import Session from "../../model/api/Session";
import TableComponent from "./TableComponent";

interface Props {
  mesa: Mesa;
  onDetails: (session: Session) => void;
  onBill: (session: Session) => void;
  newSession: (mesa: Mesa) => void;
  endSession: (session: Session) => void;
  removeTable: (mesa: Mesa) => void;
};

const TableComponentWrapper = ({
  mesa,
  onDetails,
  onBill,
  newSession,
  endSession,
  removeTable
}: Props) => {
  return <>
    <hr/>
    <TableComponent
      mesa={mesa}
      onDetails={onDetails}
      onBill={onBill}
      endSession={endSession}
      newSession={newSession}
      removeTable={removeTable}
    />
  </>;
};

export default TableComponentWrapper;