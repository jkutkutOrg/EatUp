import Session from "../../model/api/Session";
import StaffAPI from "../../services/StaffApi";

interface Props {
  session: Session;
}

const Details = ({session}: Props) => {
  console.log(session);

  const localSessionStr = localStorage.getItem(session.id);

  if (localSessionStr == null) {
    return <p>No info for this session</p>;
  }
  const localSession = JSON.parse(localSessionStr);
  const simple_id = localSession.simple_id;
  const qr_code = StaffAPI.getQR(localSession.qr_img);
  return (
    <div className="container text-center">
      <h2>Mesa {session.table_id}</h2>
      <h5>{session.id}</h5>
      <img src={qr_code} alt={session.id} style={{
        width:  "150px",
        height: "150px"
      }}/>
      <h5>{simple_id}</h5>
    </div>
  );
}

export default Details;