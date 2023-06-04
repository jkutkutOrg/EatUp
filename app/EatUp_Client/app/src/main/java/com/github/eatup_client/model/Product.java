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

    public Product(String id, String name, String description, String img_id, double price, List<Allergy> allergies, List<Category> categories) {
        this.id = id;
        this.name = name;
        this.description = description;
        this.img_id = img_id;
        this.price = price;
        this.allergies = allergies;
        this.categories = categories;
    }


    public String getId() {
        return id;
    }

    public void setId(String id) {
        this.id = id;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getDescription() {
        return description;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    public String getImg_id() {
        return img_id;
    }

    public void setImg_id(String img_id) {
        this.img_id = img_id;
    }

    public double getPrice() {
        return price;
    }

    public void setPrice(double price) {
        this.price = price;
    }

    public List<Allergy> getAllergies() {
        return allergies;
    }

    public void setAllergies(List<Allergy> allergies) {
        this.allergies = allergies;
    }

    public List<Category> getCategories() {
        return categories;
    }

    public void setCategories(List<Category> categories) {
        this.categories = categories;
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