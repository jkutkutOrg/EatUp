import { useEffect, useState } from "react";
import Session from "../../model/api/Session";
import Details from "./Details"
import StaffAPI from "../../services/StaffApi";
import Order from "../../model/api/Order";
import OrderComponent from "../../components/bill/OrderComponent";
import Price from "../../components/bill/Price";

interface Props {
  session: Session;
}

const Bill = ({session}: Props) => {
  const [orders, setOrders] = useState<Order[] | null>(null);

  useEffect(() => {
    StaffAPI.getOrders(
      session.id,
      (orders) => {
        setOrders(Order.fromJSONArray(orders));
      }
    );
  }, []);

  if (orders == null) {
    return <p>Loading...</p>;
  }

  const total = orders.reduce((acc, order) => {
    return acc + order.products.reduce((acc, product) => {
      return acc + product.quantity * product.product.price;
    }, 0);
  }, 0);

  return <>
  <Details session={session} />
  <br />
  {orders.length == 0 && <p>No orders</p>}
  {orders.length > 0 && orders.map((order, index) =>
    <div key={index}>
      <hr />
      <OrderComponent index={index} order={order}/>
    </div>
  )}
  <hr/>
  <div className="container text-end">
    <div className="row">
      <div className="col-11">
        Total <Price price={total}/>
      </div>
    </div>
  </div>
  <br />
  <br />
  </>;
}

export default Bill;