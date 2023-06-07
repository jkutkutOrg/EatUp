import Session from "../../model/api/Session";
import StaffAPI from "../../services/StaffApi";

interface Props {
  session: Session;
}

const Details = ({session}: Props) => {
  const localSessionStr = localStorage.getItem(session.id);

  if (localSessionStr == null) {
    return <div className="container text-center">
      <h2>Table {session.table_id}</h2>
      <h5>{session.id}</h5>
    </div>;
  }
  const localSession = JSON.parse(localSessionStr);
  const simple_id = localSession.simple_id;
  const qr_code = StaffAPI.getQR(localSession.qr_img);
  return (
    <div className="container text-center">
      <br />
      <h2>Table {session.table_id}</h2>
      <br />
      <h5>{session.id}</h5>
      <img src={qr_code} alt={session.id} style={{
        width:  "200px",
        height: "200px"
      }}/>
      <h5>{simple_id}</h5>
    </div>
  );
}

export default Details;