import Service from "../../model/Service";
import Services from "./Services";

interface Props {
    services: Service[],
}

const ServicesHandler = ({services}: Props) => {
    return <div className="mt-5 px-lg-5 container">
        <h1 className="mb-3">Services</h1>
        <Services services={services}/>
    </div>
}

export default ServicesHandler;