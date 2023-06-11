class Product {

  private id: string;
  public name: string;
  private description: string;
  private img_id: string;
  public price: number;
  private allergies: Allergy[];
  private categories: Category[];

  constructor(
    id: string,
    name: string,
    description: string,
    img_id: string,
    price: number,
    allergies: Allergy[],
    categories: Category[]
  ) {
    this.id = id;
    this.name = name;
    this.description = description;
    this.img_id = img_id;
    this.price = price;
    this.allergies = allergies;
    this.categories = categories;
  }

  public static fromJSON(json: any): Product {
    return new Product(
      json.id,
      json.name,
      json.description,
      json.img_id,
      json.price,
      Allergy.fromJSONArray(json.allergies),
      Category.fromJSONArray(json.categories)
    );
  }

  public static fromJSONArray(array: any[]): Product[] {
    return array.map((obj) => Product.fromJSON(obj));
  }
}

class Allergy {
  private id: string;
  private name: string;

  constructor(
    id: string,
    name: string,
  ) {
    this.id = id;
    this.name = name;
  }

  private static fromJSON(json: any): Allergy {
    return new Allergy(
      json.id,
      json.name,
    );
  }

  public static fromJSONArray(array: any[]): Allergy[] {
    return array.map((obj) => Allergy.fromJSON(obj));
  }
}

class Category {
  private id: string;
  private name: string;

  constructor(
    id: string,
    name: string
  ) {
    this.id = id;
    this.name = name;
  }

  private static fromJSON(json: any): Category {
    return new Category(
      json.id,
      json.name
    );
  }

  public static fromJSONArray(array: any[]): Category[] {
    return array.map((obj) => Category.fromJSON(obj));
  }
}

export default Product;