package com.github.eatup_client.model;

public class Allergy {
    private String id;
    private String name;
    private String img_id;

    public String getId() {
        return id;
    }

    public String getName() {
        return name;
    }

    public String getImg_id() {
        return img_id;
    }

    @Override
    public String toString() {
        return "Allergy{" +
                "id='" + id + '\'' +
                ", name='" + name + '\'' +
                ", img_id='" + img_id + '\'' +
                '}';
    }
}
