import ProductOrder from './ProductOrder';

class Order {

    private id: string;
    public products: ProductOrder[];

    constructor(
        id: string,
        products: ProductOrder[]
    ) {
        this.id = id;
        this.products = products;
    }

    private static fromJSON(json: any): Order {
        return new Order(
            json.id,
            ProductOrder.fromJSONArray(json.products)
        );
    }

    public static fromJSONArray(array: any[]): Order[] {
        return array.map((obj) => Order.fromJSON(obj));
    }

    public static calcTotalPrice(orders: Order[]): number {
        return orders.reduce((acc, order) => {
            return acc + order.products.reduce((acc, product) => {
            return acc + product.quantity * product.product.price;
            }, 0);
        }, 0);
    }
}

export default Order;