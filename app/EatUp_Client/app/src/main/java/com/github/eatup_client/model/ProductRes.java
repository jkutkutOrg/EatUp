package com.github.eatup_client.model;

import java.util.ArrayList;
import java.util.List;

public class ProductRes {
    private static ProductRes instance;
    private List<OrderProduct> orderProducts;

    private ProductRes() {
        orderProducts = new ArrayList<>();
    }

    public static ProductRes getInstance() {
        if (instance == null) {
            instance = new ProductRes();
        }
        return instance;
    }

    public List<OrderProduct> getOrderProducts() {
        return orderProducts;
    }

    public void addProduct(Product product) {
        OrderProduct orderProduct = getOrderProduct(product);
        if (orderProduct != null) {
            orderProduct.setQuantity(orderProduct.getQuantity() + 1);
        } else {
            OrderProduct newOrderProduct = new OrderProduct(1, product);
            orderProducts.add(newOrderProduct);
        }
    }

    public void removeProduct(Product product) {
        OrderProduct orderProduct = getOrderProduct(product);
        if (orderProduct != null) {
            int newQuantity = orderProduct.getQuantity() - 1;
            if (newQuantity > 0) {
                orderProduct.setQuantity(newQuantity);
            } else {
                orderProducts.remove(orderProduct);
            }
        }
    }

    public OrderProduct getOrderProduct(Product product) {
        for (OrderProduct orderProduct : orderProducts) {
            if (orderProduct.getProduct().equals(product)) {
                return orderProduct;
            }
        }
        return null;
    }

    public void setQuantity(Product product, int quantity) {
        OrderProduct orderProduct = getOrderProduct(product);
        if (orderProduct != null) {
            orderProduct.setQuantity(quantity);
        } else {
            OrderProduct newOrderProduct = new OrderProduct(quantity, product);
            orderProducts.add(newOrderProduct);
        }
    }
}
