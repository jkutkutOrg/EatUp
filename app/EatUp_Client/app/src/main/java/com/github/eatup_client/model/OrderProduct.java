package com.github.eatup_client.model;

public class OrderProduct {
    private String id;
    private int quantity;
    private Product product;

    public OrderProduct(String id, int quantity, Product product) {
        this.id = id;
        this.quantity = quantity;
        this.product = product;
    }

    public String getId() {
        return id;
    }

    public int getQuantity() {
        return quantity;
    }

    public Product getProduct() {
        return product;
    }

    @Override
    public String toString() {
        return "OrderProduct{" +
                "id='" + id + '\'' +
                ", quantity=" + quantity +
                ", product=" + product +
                '}';
    }
}
