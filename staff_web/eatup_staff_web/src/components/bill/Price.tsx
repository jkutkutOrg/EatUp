interface Props {
  price: number;
};

const Price = ({price}: Props) => {
  return <>{price.toFixed(2)}€</>;
}

export default Price;