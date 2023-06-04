interface Props {
  
}

const Bill = ({}: Props) => {
  return <>
  <h1>Bill</h1>
  <h5>Session Id: XXXXXXXXX</h5>
  <div style={{
    display: "flex",
    flexDirection: "row",
    justifyContent: "space-between",
  }}>
  <span>product name</span>
  <span>product.price</span>
  <span>product.quantity</span>
  </div>
  {[
    {
      name: "Item 1",
      price: 10,
      quantity: 2,
    },
    {
      name: "Item 2",
      price: 10,
      quantity: 2,
    },
    {
      name: "Item 3",
      price: 10,
      quantity: 2,
    },
    {
      name: "Item 4",
      price: 10,
      quantity: 2,
    }
  ].map((product) => {
    return <div key={product.name} style={{
      display: "flex",
      flexDirection: "row",
      justifyContent: "space-between",
    }}>
    <span>{product.name}</span>
    <span>{product.price}</span>
    <span>{product.quantity}</span>
    </div>
  })}
  <br />
  <span>Total: 32um</span>
  </>;
}

export default Bill;