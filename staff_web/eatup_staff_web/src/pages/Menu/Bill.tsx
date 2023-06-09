import { useEffect, useState } from "react";
import Session from "../../model/api/Session";
import Details from "./Details"
import StaffAPI from "../../services/StaffApi";
import Order from "../../model/api/Order";
import TotalPrice from "../../components/bill/TotalPrice";
import OrdersComponent from "../../components/bill/Order/OrdersComponent";
import Loading from "../../components/loading/Loading";

interface Props {
  session: Session;
}

const Bill = ({session}: Props) => {
  const [orders, setOrders] = useState<Order[] | null>(null);

  const updateOrders = () => {
    StaffAPI.getOrders(
      session.id,
      (orders) => {
        setOrders(Order.fromJSONArray(orders));
      }
    );
  }; useEffect(updateOrders, []);
  window.addEventListener("focus", updateOrders);

  if (orders == null) {
    return <Loading />;
  }

  return <>
  <Details session={session} />
  <br />
  <OrdersComponent orders={orders}/>
  <hr/>
  <div className="container text-end">
    <div className="row">
      <div className="col-11">
        <TotalPrice orders={orders}/>
      </div>
    </div>
  </div>
  <br />
  <br />
  </>;
}

export default Bill;