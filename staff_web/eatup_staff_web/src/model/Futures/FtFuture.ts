interface FtFuture<T, S> {
  (data: T): Promise<S>;
}

export default FtFuture;