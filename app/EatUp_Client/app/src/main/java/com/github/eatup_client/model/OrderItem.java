package com.github.eatup_client.model;

import java.util.ArrayList;
import java.util.List;

public class OrderItem {
    private String id;
    private List<OrderProduct> products;

    private static List<OrderItem> orderItems = new ArrayList<>();

    public OrderItem(String id, List<OrderProduct> products) {
        this.id = id;
        this.products = products;
    }

    public String getId() {
        return id;
    }

    public List<OrderProduct> getProducts() {
        return products;
    }

    public static List<OrderItem> getOrderItems() {
        return orderItems;
    }

    public static void setOrderItems(List<OrderItem> orderItems) {
        OrderItem.orderItems = orderItems;
    }

    public static List<Product> getCartProducts() {
        List<Product> cartProducts = new ArrayList<>();
        for (OrderItem orderItem : orderItems) {
            List<OrderProduct> products = orderItem.getProducts();
            for (OrderProduct orderProduct : products) {
                cartProducts.add(orderProduct.getProduct());
            }
        }
        return cartProducts;
    }

    @Override
    public String toString() {
        return "OrderItem{" +
                "id='" + id + '\'' +
                ", products=" + products +
                '}';
    }
}
