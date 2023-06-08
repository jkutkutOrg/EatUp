package com.github.eatup_client.model;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class ProductRes {
    private static ProductRes instance;
    private Map<Product, OrderProduct> orderProductMap;
    private Map<String, Session> sessionMap;
    private Map<String, Order> orderMap;

    private ProductRes() {
        orderProductMap = new HashMap<>();
        sessionMap = new HashMap<>();
        orderMap = new HashMap<>();
    }

    public static synchronized ProductRes getInstance() {
        if (instance == null) {
            instance = new ProductRes();
        }
        return instance;
    }

    public List<OrderProduct> getOrderProducts() {
        return new ArrayList<>(orderProductMap.values());
    }

    public void addProduct(Product product) {
        OrderProduct orderProduct = orderProductMap.get(product);
        if (orderProduct != null) {
            orderProduct.setQuantity(orderProduct.getQuantity() + 1);
        } else {
            OrderProduct newOrderProduct = new OrderProduct(1, product);
            orderProductMap.put(product, newOrderProduct);
        }
    }

    public double getTotalPrice() {
        double totalPrice = 0;
        for (OrderProduct orderProduct : orderProductMap.values()) {
            totalPrice += orderProduct.getQuantity() * orderProduct.getProduct().getPrice();
        }
        return totalPrice;
    }

    // Remove all OrderProducts
    public void clear() {
        orderProductMap.clear();
    }

    public OrderProduct getOrderProduct(Product product) {
        return orderProductMap.get(product);
    }

    public void setQuantity(Product product, int quantity) {
        if (quantity <= 0) {
            orderProductMap.remove(product);
        } else {
            OrderProduct orderProduct = orderProductMap.get(product);
            if (orderProduct != null) {
                orderProduct.setQuantity(quantity);
            } else {
                OrderProduct newOrderProduct = new OrderProduct(quantity, product);
                orderProductMap.put(product, newOrderProduct);
            }
        }
    }

    // Sesions
    public void addSession(Session session) {
        sessionMap.put(session.getId(), session);
    }

    public Session getSession(String id) {
        return sessionMap.get(id);
    }

    // Get session UUID
    public String getSessionId() {
        return sessionMap.keySet().iterator().next();
    }

    // Orders
    public void addOrder(Order order) {
        orderMap.put(order.getSessionId(), order);
    }

    public Order getOrder(String sessionId) {
        return orderMap.get(sessionId);
    }
}