import FtCreate from "../model/FtApiActions/FtCreate";

interface Props {
    ftCreate: FtCreate;
}

const Start = ({ftCreate}: Props) => {
  return <>
    <div className="container mt-5">
      <div className="row text-center">
        <div className="card-body">
          <h2 className="card-title">Welcome to EatUp!</h2>
          <br/>
          <p className="card-text">This is the setup webpage, where you will be able to handle all the backend of this project.</p>
          <br/>
          <button className="btn btn-primary btn-lg" onClick={ftCreate}>Let's start!</button>
        </div>
      </div>
    </div>
  </>;
}

export default Start;