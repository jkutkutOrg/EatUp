const Loading = () => {
  return (
    <div className="text-center mt-5">
      <div className="spinner-border text-light" role="status" style={{
      width: "3rem",
      height: "3rem",
    }}>
        <span className="sr-only"></span>
      </div>
    </div>
  );
};

export default Loading;