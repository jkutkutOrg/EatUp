package com.github.eatup_client.model;

import java.util.List;

public class Order {
    private List<OrderProduct> products;
    private String sessionId;

    public Order(List<OrderProduct> products, String sessionId) {
        this.products = products;
        this.sessionId = sessionId;
    }

    public List<OrderProduct> getProducts() {
        return products;
    }

    public String getSessionId() {
        return sessionId;
    }
}