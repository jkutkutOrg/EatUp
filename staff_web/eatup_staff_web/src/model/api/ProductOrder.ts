import Product from "./Product";

class ProductOrder {
  private id: string;
  public quantity: number;
  public product: Product;

  constructor(
    id: string,
    quantity: number,
    product: Product
  ) {
    this.id = id;
    this.quantity = quantity;
    this.product = product;
  }

  private static fromJSON(json: any): ProductOrder {
    return new ProductOrder(
      json.id,
      json.quantity,
      Product.fromJSON(json.product)
    );
  }

  public static fromJSONArray(array: any[]): ProductOrder[] {
    return array.map((obj) => ProductOrder.fromJSON(obj));
  }
}

export default ProductOrder;