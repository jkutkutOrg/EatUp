import Services from "./Services";

function ServicesHandler() {
    return <div className="mt-5 px-lg-5 container">
        <h1 className="mb-3">Services</h1>
        <Services services={[
            "eatup_db",
            "eatup_server",
        ]}/>
    </div>
}

export default ServicesHandler;