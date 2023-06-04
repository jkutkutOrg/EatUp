package com.github.eatup_client.model;

public class OrderProduct {
    private int quantity;
    private Product product;

    public OrderProduct(int quantity, Product product) {
        this.quantity = quantity;
        this.product = product;
    }


    public int getQuantity() {
        return quantity;
    }

    public void setQuantity(int quantity) {
        this.quantity = quantity;
    }

    public Product getProduct() {
        return product;
    }

    @Override
    public String toString() {
        return "OrderProduct{" +
                ", quantity=" + quantity +
                ", product=" + product +
                '}';
    }
}
