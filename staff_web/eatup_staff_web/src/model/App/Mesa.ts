import Session from "../api/Session"

class Mesa {
  private name: string;
  private session: Session | null;

  constructor(name: string) {
    this.name = name;
    this.session = null;
  }

  public setSession(session: Session) {
    this.session = session;
  }

  public getName(): string {
    return this.name;
  }

  public getSession(): Session | null {
    return this.session;
  }
}

export default Mesa;