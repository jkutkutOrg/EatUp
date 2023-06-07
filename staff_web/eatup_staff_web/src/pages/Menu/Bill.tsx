import { useEffect, useState } from "react";
import Session from "../../model/api/Session";
import Details from "./Details"
import StaffAPI from "../../services/StaffApi";
import Order from "../../model/api/Order";

interface Props {
  session: Session;
}

const priceFormat = (price: number) => {
  return price.toFixed(2) + "€";
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
      <hr/>
      <div className="container text-center">
        <div className="row">
          <div className="col">
            <h5>Order {index + 1}</h5>
          </div>
        </div>
        <div className="row">
          <div className="col-6"><b>Product</b></div>
          <div className="col-3"><b>Quantity</b></div>
          <div className="col-3"><b>Unit price</b></div>
        </div>
        {order.products.map((product) => {
          return <div key={product.product.name} className="row">
            <div className="col-6">{product.product.name}</div>
            <div className="col-3">{product.quantity}</div>
            <div className="col-3">{priceFormat(product.product.price)}</div>
          </div>;
        })}
        <br />
        <div className="row text-end">
          <div className="col-11">Subtotal {priceFormat(
            order.products.reduce((acc, product) => {
              return acc + product.quantity * product.product.price;
            }, 0)
          )}</div>
        </div>
      </div>
      <br />
    </div>
  )}
  <hr/>
  <div className="container text-end">
    <div className="row">
      <div className="col-11">Total {priceFormat(total)}</div>
    </div>
  </div>
  <br />
  <br />
  </>;
}

export default Bill;