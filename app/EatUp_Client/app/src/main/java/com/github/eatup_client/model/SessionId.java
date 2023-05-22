package com.github.eatup_client.model;

public class SessionId {
    private String simpleId;
    private String id;
    private String qrImg;

    public SessionId(String simpleId, String id, String qrImg) {
        this.simpleId = simpleId;
        this.id = id;
        this.qrImg = qrImg;
    }

    public String getSimpleId() {
        return simpleId;
    }

    public String getId() {
        return id;
    }

    public String getQrImg() {
        return qrImg;
    }

    @Override
    public String toString() {
        return "SessionId{" +
                "simpleId='" + simpleId + '\'' +
                ", id='" + id + '\'' +
                ", qrImg='" + qrImg + '\'' +
                '}';
    }
}
