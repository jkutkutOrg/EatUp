package com.github.eatup_client.model;

public class Session {
    private String id;
    private String tableId;
    private boolean inProgress;

    public Session(String id, String tableId, boolean inProgress) {
        this.id = id;
        this.tableId = tableId;
        this.inProgress = inProgress;
    }

    public String getId() {
        return id;
    }

    public String getTableId() {
        return tableId;
    }

    public boolean isInProgress() {
        return inProgress;
    }

    @Override
    public String toString() {
        return "Session{" +
                "id='" + id + '\'' +
                ", tableId='" + tableId + '\'' +
                ", inProgress=" + inProgress +
                '}';
    }
}