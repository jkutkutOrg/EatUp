package com.github.eatup_client.model;

import com.google.gson.annotations.Expose;
import com.google.gson.annotations.SerializedName;

import java.io.Serializable;

public class Session implements Serializable {
    @SerializedName("id")
    @Expose
    private String id;
    @SerializedName("table_id")
    @Expose
    private String tableId;
    @SerializedName("in_progress")
    @Expose
    private boolean inProgress;

    // Constructor
    public Session(String id, String tableId, boolean inProgress) {
        this.id = id;
        this.tableId = tableId;
        this.inProgress = inProgress;
    }

    // Getters
    public String getId() {
        return id;
    }

    public String getTableId() {
        return tableId;
    }

    public boolean isInProgress() {
        return inProgress;
    }

    // ToString method for debugging
    @Override
    public String toString() {
        return "Session{" +
                "id='" + id + '\'' +
                ", tableId='" + tableId + '\'' +
                ", inProgress=" + inProgress +
                '}';
    }
}