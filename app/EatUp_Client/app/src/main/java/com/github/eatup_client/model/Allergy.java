package com.github.eatup_client.model;

import com.google.gson.annotations.Expose;
import com.google.gson.annotations.SerializedName;

import java.io.Serializable;

public class Allergy implements Serializable {

    @SerializedName("id")
    @Expose
    private String id;
    @SerializedName("name")
    @Expose
    private String name;
    @SerializedName("img_id")
    @Expose
    private String img_id;

    // Constructor
    public Allergy(String id, String name, String img_id) {
        this.id = id;
        this.name = name;
        this.img_id = img_id;
    }

    // Getters
    public String getId() {
        return id;
    }

    public String getName() {
        return name;
    }

    public String getImg_id() {
        return img_id;
    }

    // ToString method for debugging
    @Override
    public String toString() {
        return "Allergy{" +
                "id='" + id + '\'' +
                ", name='" + name + '\'' +
                ", img_id='" + img_id + '\'' +
                '}';
    }
}