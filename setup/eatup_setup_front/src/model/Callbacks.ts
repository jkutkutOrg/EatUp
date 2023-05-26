interface InstallationCallback {
    (dbUsr: string, dbUsrPasswd: string, serverPort: number): void;
}

export default InstallationCallback;