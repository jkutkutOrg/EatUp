package com.github.eatup_client.model;

import java.util.List;

public class Product {
    private String id;
    private String name;
    private String description;
    private String img_id;
    private double price;
    private List<Allergy> allergies;
    private List<Category> categories;

    public String getId() {
        return id;
    }

    public String setId(String id) {
        return this.id = id;
    }

    public String getName() {
        return name;
    }

    public String getDescription() {
        return description;
    }

    public String getImg_id() {
        return img_id;
    }

    public double getPrice() {
        return price;
    }

    public List<Allergy> getAllergies() {
        return allergies;
    }

    public List<Category> getCategories() {
        return categories;
    }

    @Override
    public String toString() {
        return "Product{" +
                "id='" + id + '\'' +
                ", name='" + name + '\'' +
                ", description='" + description + '\'' +
                ", img_id='" + img_id + '\'' +
                ", price=" + price +
                ", allergies=" + allergies +
                ", categories=" + categories +
                '}';
    }
}