import Service from "../../model/Service";

interface Props {
  services: Service[];
}

const Services = ({services}: Props) => {
  return (
    <ul className="list-group">
      {services.map((service, index) => (
        <li 
          key={index}
          className="list-group-item"
        >
          <div>
            <div className="d-flex justify-content-between">
              <h5 className="mb-1">{service.name}</h5>
              <small>{service.state}</small>
            </div>
          </div>
        </li>
      ))}
    </ul>
  );
}

export default Services;