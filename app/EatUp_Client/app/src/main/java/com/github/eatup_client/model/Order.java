package com.github.eatup_client.model;

import com.google.gson.annotations.Expose;
import com.google.gson.annotations.SerializedName;

import java.io.Serializable;
import java.util.List;

public class Order implements Serializable {

    @SerializedName("products")
    @Expose
    private List<OrderProduct> products;
    @SerializedName("session_id")
    @Expose
    private String sessionId;

    // Constructor
    public Order(List<OrderProduct> products, String sessionId) {
        this.products = products;
        this.sessionId = sessionId;
    }

    // Getters
    public List<OrderProduct> getProducts() {
        return products;
    }

    public String getSessionId() {
        return sessionId;
    }

    // ToString method for debugging
    @Override
    public String toString() {
        return "Order{" +
                "products=" + products +
                ", sessionId='" + sessionId + '\'' +
                '}';
    }
}