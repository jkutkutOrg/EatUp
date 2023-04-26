interface Props {
  services: string[];
}

function Services({services}: Props) {
  return (
    <ul className="list-group">
      {services.map((service, index) => (
        <li 
          key={index}
          className="list-group-item"
        >
          {service}
        </li>
      ))}
    </ul>
  );
}

export default Services;