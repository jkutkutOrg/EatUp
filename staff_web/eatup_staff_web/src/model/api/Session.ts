class Session {
  id: string;
  table_id: string;
  in_progress: boolean;

  constructor(id: string, table_id: string, in_progress: boolean) {
    this.id = id;
    this.table_id = table_id;
    this.in_progress = in_progress;
  }

  private static fromJson(json: any): Session {
    return new Session(json.id, json.table_id, json.in_progress);
  }

  public static fromJsonArray(json: any[]): Session[] {
    return json.map((session: any) => Session.fromJson(session));
  }
}

export default Session;