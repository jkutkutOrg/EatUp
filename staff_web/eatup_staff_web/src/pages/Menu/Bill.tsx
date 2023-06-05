import { useEffect, useState } from "react";
import Session from "../../model/api/Session";
import Details from "./Details"

interface Props {
  session: Session;
}

const priceFormat = (price: number) => {
  return price.toFixed(2) + "€";
}

const Bill = ({session}: Props) => {
  const [orders, setOrders] = useState<any[] | null>(null);
  // const [orders, setOrders] = useState<Order[] | null>(null);

  useEffect(() => {
    // setOrders([
    //   {
    //     "id": "AAAAAAA",
    //     "products": [
    //       {
    //         "id": "BBBBBBB",
    //         "quantity": 2,
    //         "product": {
    //           "id": "CCCCCCC",
    //           "name": "Bruschetta",
    //           "description": "Tomato, garlic, basil, olive oil",
    //           "img_id": "bruchetta.png",
    //           "price": 5.0,
    //           "allergies": [
    //             {
    //               "id": "DDDDDDD",
    //               "name": "Gluten",
    //               "img_id": "gluten.png"
    //             },
    //             {
    //               "id": "EEEEEEE",
    //               "name": "Lactose",
    //               "img_id": "lactose.png"
    //             }
    //           ],
    //           "categories": [
    //             {
    //               "id": "FFFFFFF",
    //               "name": "Appetizers"
    //             }
    //           ]
    //         }
    //       },
    //       {
    //         "id": "GGGGGGG",
    //         "quantity": 1,
    //         "product": {
    //           "id": "HHHHHHH",
    //           "name": "Margherita",
    //           "description": "Tomato, mozzarella, basil",
    //           "img_id": "margherita.png",
    //           "price": 7.0,
    //           "allergies": [
    //             {
    //               "id": "IIIIIII",
    //               "name": "Gluten",
    //               "img_id": "gluten.png"
    //             },
    //             {
    //               "id": "JJJJJJJ",
    //               "name": "Lactose",
    //               "img_id": "lactose.png"
    //             }
    //           ],
    //           "categories": [
    //             {
    //               "id": "KKKKKKK",
    //               "name": "Pizzas"
    //             }
    //           ]
    //         }
    //       }
    //     ],
    //   },
    //   {
    //     "id": "AAAAAAA",
    //     "products": [
    //       {
    //         "id": "BBBBBBB",
    //         "quantity": 2,
    //         "product": {
    //           "id": "CCCCCCC",
    //           "name": "Bruschetta",
    //           "description": "Tomato, garlic, basil, olive oil",
    //           "img_id": "bruchetta.png",
    //           "price": 5.0,
    //           "allergies": [
    //             {
    //               "id": "DDDDDDD",
    //               "name": "Gluten",
    //               "img_id": "gluten.png"
    //             },
    //             {
    //               "id": "EEEEEEE",
    //               "name": "Lactose",
    //               "img_id": "lactose.png"
    //             }
    //           ],
    //           "categories": [
    //             {
    //               "id": "FFFFFFF",
    //               "name": "Appetizers"
    //             }
    //           ]
    //         }
    //       },
    //       {
    //         "id": "GGGGGGG",
    //         "quantity": 1,
    //         "product": {
    //           "id": "HHHHHHH",
    //           "name": "Margherita",
    //           "description": "Tomato, mozzarella, basil",
    //           "img_id": "margherita.png",
    //           "price": 7.0,
    //           "allergies": [
    //             {
    //               "id": "IIIIIII",
    //               "name": "Gluten",
    //               "img_id": "gluten.png"
    //             },
    //             {
    //               "id": "JJJJJJJ",
    //               "name": "Lactose",
    //               "img_id": "lactose.png"
    //             }
    //           ],
    //           "categories": [
    //             {
    //               "id": "KKKKKKK",
    //               "name": "Pizzas"
    //             }
    //           ]
    //         }
    //       },
    //       {
    //         "id": "BBBBBBB",
    //         "quantity": 3,
    //         "product": {
    //           "id": "CCCCCCC",
    //           "name": "Beer",
    //           "description": "Beer",
    //           "img_id": "beer.png",
    //           "price": 15.0,
    //           "allergies": [
    //             {
    //               "id": "DDDDDDD",
    //               "name": "Gluten",
    //               "img_id": "gluten.png"
    //             },
    //             {
    //               "id": "EEEEEEE",
    //               "name": "Lactose",
    //               "img_id": "lactose.png"
    //             }
    //           ],
    //           "categories": [
    //             {
    //               "id": "FFFFFFF",
    //               "name": "Drinks"
    //             }
    //           ]
    //         }
    //       }
    //     ],
    //   },
    // ]);


    // setOrders([]);
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
    <>
      <h5>Order {index + 1}</h5>
      <div key={index} className="container text-center">
        <div className="row">
          <div className="col"><b>Product</b></div>
          <div className="col"><b>Quantity</b></div>
          <div className="col"><b>Unit price</b></div>
        </div>
        {order.products.map((product) => {
          return <div key={product.product.name} className="row">
            <div className="col">{product.product.name}</div>
            <div className="col">{product.quantity}</div>
            <div className="col">{priceFormat(product.product.price)}</div>
          </div>;
        })}
      </div>
      <br />
    </>
  )}
  <div className="container text-end">
    <div className="row">
      <div className="col-11">Total {priceFormat(total)}</div>
    </div>
  </div>


  </>;
}

export default Bill;