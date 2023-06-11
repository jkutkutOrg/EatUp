package com.github.eatup_client.model;

import com.google.gson.annotations.Expose;
import com.google.gson.annotations.SerializedName;

import java.io.Serializable;

public class OrderProduct implements Serializable {

    @SerializedName("quantity")
    @Expose
    private int quantity;
    @SerializedName("product")
    @Expose
    private Product product;

    // Constructor
    public OrderProduct(int quantity, Product product) {
        this.quantity = quantity;
        this.product = product;
    }

    // Getters
    public int getQuantity() {
        return quantity;
    }

    public Product getProduct() {
        return product;
    }

    // Setters
    public void setQuantity(int quantity) {
        this.quantity = quantity;
    }

    // ToString method for debugging
    @Override
    public String toString() {
        return "OrderProduct{" +
                ", quantity=" + quantity +
                ", product=" + product +
                '}';
    }
}