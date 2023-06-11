import MicroService from "../../model/Microservices/MicroService";
import MicroServiceComponent from "./MicroServiceContainer";

interface Props {
  microservices: MicroService[];
  ftStartMicroservice: (name: string) => void;
  ftStopMicroservice: (name: string) => void;
}

const MicroServicesContainer = ({microservices, ftStartMicroservice, ftStopMicroservice}: Props) => {
  return <div className="mt-5 px-lg-5 container">
    <table className="table table-hover table-dark">
    <thead>
      <tr>
        <th scope="col">Microservice</th>
        <th scope="col">Container id</th>
        <th scope="col">Status</th>
        <th scope="col">IP</th>
        <th scope="col text-center">Port</th>
        <th scope="col text-center"></th>
      </tr>
    </thead>
      <tbody>
        {microservices.map((service, index) => (
          <MicroServiceComponent
            key={index}
            service={service}
            ftStartMicroservice={ftStartMicroservice}
            ftStopMicroservice={ftStopMicroservice}
          />
        ))}
      </tbody>
    </table>
  </div>;
}

export default MicroServicesContainer;