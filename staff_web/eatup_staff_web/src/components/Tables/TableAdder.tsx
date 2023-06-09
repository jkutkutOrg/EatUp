import { useState } from "react";
import Mesa from "../../model/App/Mesa";
import EatupButton from "../btn/EatupButton";
import Modal from 'react-bootstrap/Modal';

interface Props {
  onAdd: (mesa: Mesa) => string | null;
};

const TableAdder = ({onAdd}: Props) => {
  const [modal, setModal] = useState<boolean>(false);

  const enableModal = () => setModal(true);
  const disableModal = () => setModal(false);

  const addTable = () => {
    const input = document.getElementById("tableInput") as HTMLInputElement;
    const mesa = new Mesa(input.value.trim());
    if (mesa.getName() == "") {
      input.classList.add("is-invalid");
      return;
    }
    input.classList.remove("is-invalid");
    const error = onAdd(mesa);
    if (error != null) {
      input.classList.add("is-invalid");
      const feedback = document.getElementById("tableInputFeedback") as HTMLDivElement;
      feedback.innerText = error;
      return;
    }
    disableModal();
  };
  
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
        {/* <input id="tableInput" className="form-control w-100" placeholder="Table name" type="text"/> */}
        <div className="input-group has-validation">
          <span className="input-group-text" id="inputGroupPrepend">Table:</span>
          <input id="tableInput" type="text" placeholder="example: 11"
            className="form-control" aria-describedby="inputGroupPrepend"
            required
          />
          <div id="tableInputFeedback" className="invalid-feedback">
            Please enter a table name.
          </div>
        </div>

      </Modal.Body>
      <Modal.Footer>
        <div className="container">
          <div className="row justify-content-end">
            <div className="col-3">
              <EatupButton type="danger" onClick={disableModal}>Cancel</EatupButton>
            </div>
            <div className="col-3">
              <EatupButton type="success" onClick={addTable}>Add</EatupButton>
            </div>
          </div>
        </div>
      </Modal.Footer>
    </Modal>
  </>);
};

export default TableAdder;