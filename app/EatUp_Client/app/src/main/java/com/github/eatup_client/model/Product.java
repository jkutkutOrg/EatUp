package com.github.eatup_client.model;

import com.google.gson.annotations.Expose;
import com.google.gson.annotations.SerializedName;

import java.io.Serializable;
import java.util.List;

public class Product implements Serializable {

    @SerializedName("id")
    @Expose
    private String id;
    @SerializedName("name")
    @Expose
    private String name;
    @SerializedName("description")
    @Expose
    private String description;
    @SerializedName("img_id")
    @Expose
    private String img_id;
    @SerializedName("price")
    @Expose
    private double price;
    @SerializedName("allergies")
    @Expose
    private List<Allergy> allergies;
    @SerializedName("categories")
    @Expose
    private List<Category> categories;

    // Constructor
    public Product(String id, String name, String description, String img_id, double price, List<Allergy> allergies, List<Category> categories) {
        this.id = id;
        this.name = name;
        this.description = description;
        this.img_id = img_id;
        this.price = price;
        this.allergies = allergies;
        this.categories = categories;
    }

    // Getters
    public String getId() {
        return id;
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

    // Setters
    public void setId(String id) {
        this.id = id;
    }

    public void setName(String name) {
        this.name = name;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    public void setImg_id(String img_id) {
        this.img_id = img_id;
    }

    public void setPrice(double price) {
        this.price = price;
    }

    public void setAllergies(List<Allergy> allergies) {
        this.allergies = allergies;
    }

    public void setCategories(List<Category> categories) {
        this.categories = categories;
    }

    // ToString method for debugging
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