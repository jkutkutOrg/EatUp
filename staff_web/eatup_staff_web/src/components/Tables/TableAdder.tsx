import { useState } from "react";
import Mesa from "../../model/App/Mesa";
import EatupButton from "../btn/EatupButton";
import Modal from 'react-bootstrap/Modal';

interface Props {
  onAdd: (mesa: Mesa) => string | null;
};

const TableAdder = ({}: Props) => {
  const [modal, setModal] = useState<boolean>(false);

  const enableModal = () => setModal(true);
  const disableModal = () => setModal(false);
  
  return (<>
    <div className="container">
      <div className="row justify-content-end">
        <div className="col-2">
          <EatupButton type="success" onClick={enableModal}>+</EatupButton>
        </div>
      </div>
    </div>
    <Modal
      show={modal}
      onHide={disableModal}
      keyboard={true}
    >
      <Modal.Header closeButton>
        <Modal.Title>Add Table</Modal.Title>
      </Modal.Header>
      <Modal.Body>
        <input id="tableInput" type="text" placeholder="Table name" />
      </Modal.Body>
      <Modal.Footer>
        <div className="container">
          <div className="row justify-content-end">
            <div className="col-3">
              <EatupButton type="danger" onClick={disableModal}>Cancel</EatupButton>
            </div>
            <div className="col-3">
              <EatupButton type="success" onClick={disableModal}>Add</EatupButton>
            </div>
          </div>
        </div>
      </Modal.Footer>
    </Modal>
  </>);
};

export default TableAdder;