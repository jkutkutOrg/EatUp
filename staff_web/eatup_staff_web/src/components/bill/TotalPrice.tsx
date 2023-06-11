import Order from "../../model/api/Order";
import Price from "./Price";

interface Props {
  title?: string;
  orders: Order[];
}

const TotalPrice = ({title = "Total:", orders}: Props) => {
  const total = Order.calcTotalPrice(orders);
  return <>{title} <Price price={total}/></>;
}

export default TotalPrice;
