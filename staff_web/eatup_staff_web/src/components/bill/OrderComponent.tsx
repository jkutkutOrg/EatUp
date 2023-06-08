import Order from "../../model/api/Order";
import Price from "./Price";

interface Props {
  index: number;
  order: Order;
};

const OrderComponent = ({order, index}: Props) => {
  const subTotal = order.products.reduce((acc, product) => {
    return acc + product.quantity * product.product.price;
  }, 0);

  return <>
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
          <div className="col-3">
            <Price price={product.product.price}/>
          </div>
        </div>;
      })}
      <br />
      <div className="row text-end">
        <div className="col-11">
          Subtotal <Price price={subTotal}/>
        </div>
      </div>
    </div>
    <br />
  </>;
}

export default OrderComponent;