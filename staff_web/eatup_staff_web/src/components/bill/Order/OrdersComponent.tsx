import Order from "../../../model/api/Order";
import OrderComponent from "./OrderComponent";

interface Props {
  orders: Order[];
}

const OrdersComponent = ({orders}: Props) => {
  if (orders.length == 0) {
    return <div className="container text-center">
      <h5>No orders made</h5>
    </div>;
  }
  return <>
    {orders.map((order, index) =>
      <div key={index}>
        <hr />
        <OrderComponent index={index} order={order}/>
      </div>
    )}
  </>;
}

export default OrdersComponent;