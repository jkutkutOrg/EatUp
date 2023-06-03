package com.github.eatup_client.model;

import java.util.List;

public class Order {
    private List<OrderProduct> products;
    private String sessionId;

    public Order(List<OrderProduct> products) {
        this.products = products;
    }

    public String getSessionId() {
        return sessionId;
    }

    @Override
    public String toString() {
        return "Order{" +
                "products=" + products +
                ", sessionId='" + sessionId + '\'' +
                '}';
    }
}