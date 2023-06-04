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

    public void setOrderProducts(List<OrderProduct> orderProducts) {
        this.orderProducts = orderProducts;
    }

    public void addProduct(Product product) {
        for (OrderProduct orderProduct : orderProducts) {
            if (orderProduct.getProduct().equals(product)) {
                orderProduct.setQuantity(orderProduct.getQuantity() + 1);
                return;
            }
        }
        OrderProduct newOrderProduct = new OrderProduct(1, product);
        orderProducts.add(newOrderProduct);
    }

    public void removeProduct(Product product) {
        for (OrderProduct orderProduct : orderProducts) {
            if (orderProduct.getProduct().equals(product)) {
                int newQuantity = orderProduct.getQuantity() - 1;
                if (newQuantity > 0) {
                    orderProduct.setQuantity(newQuantity);
                } else {
                    orderProducts.remove(orderProduct);
                }
                return;
            }
        }
    }

    public int getQuantity(Product product) {
        for (OrderProduct orderProduct : orderProducts) {
            if (orderProduct.getProduct().equals(product)) {
                return orderProduct.getQuantity();
            }
        }
        return 0;
    }
}
