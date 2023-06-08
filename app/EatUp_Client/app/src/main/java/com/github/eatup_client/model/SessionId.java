package com.github.eatup_client.model;

import com.google.gson.annotations.Expose;
import com.google.gson.annotations.SerializedName;

import java.io.Serializable;

public class SessionId implements Serializable {
    @SerializedName("simple_id")
    @Expose
    private String simpleId;
    @SerializedName("id")
    @Expose
    private String id;
    @SerializedName("qr_img")
    @Expose
    private String qrImg;

    // Constructor
    public SessionId(String simpleId, String id, String qrImg) {
        this.simpleId = simpleId;
        this.id = id;
        this.qrImg = qrImg;
    }

    // Getters
    public String getSimpleId() {
        return simpleId;
    }

    public String getId() {
        return id;
    }

    public String getQrImg() {
        return qrImg;
    }

    // ToString method for debugging
    @Override
    public String toString() {
        return "SessionId{" +
                "simpleId='" + simpleId + '\'' +
                ", id='" + id + '\'' +
                ", qrImg='" + qrImg + '\'' +
                '}';
    }
}
