import { useState } from "react";
import Mesa from "../../model/App/Mesa";
import Session from "../../model/api/Session";
import EatupButton from "../btn/EatupButton";
import TableComponentWrapper from "./TableComponentWrapper";

interface Props {
  untrackedTables: Mesa[];
  onDetails: (session: Session) => void;
  onBill: (session: Session) => void;
  endSession: (session: Session) => void;
};

const UntrackedTables = ({untrackedTables, onDetails, onBill, endSession}: Props) => {
  const [showUntracked, setShowUntracked] = useState<boolean>(false);

  const show = () => setShowUntracked(true);
  const hide = () => setShowUntracked(false);

  let header;
  if (showUntracked) {
    header = <>
      <div className="col-8">
        <h2>Untracked tables</h2>
      </div>
      <div className="col-4">
        <EatupButton onClick={hide}>Hide</EatupButton>
      </div>
    </>;
  }
  else {
    header = <>
      <div className="col">
        <EatupButton onClick={show}>Show untracked tables</EatupButton>
      </div>
    </>;
  }

  if (untrackedTables.length == 0)
    return null;
  return (<>
    <br />
    <div className="container">
      <div className="row">
        {header}
      </div>
    </div>
    {showUntracked && <>
      {untrackedTables.map((mesa, i) => (
        <TableComponentWrapper key={i}
          mesa={mesa}
          onDetails={onDetails}
          onBill={onBill}
          endSession={endSession}
          newSession={() => {}}
          removeTable={() => {}}
        />
      ))}
      <hr />
    </>}
  </>);
};

export default UntrackedTables;