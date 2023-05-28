interface FtCallback<T, S> {
    (data: T): Promise<S> | void;
}

export default FtCallback;