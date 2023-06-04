interface Props {

}

const Tables = ({}: Props) => {
  let mesas = [10, 11, 12, 13];

  return <>
    <h1>Tables</h1>
    {mesas.map((mesa) => (
      <div key={mesa}>
        <h5>Mesa {mesa}</h5>
        <div style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-between",
        }}>
          <span>{mesa % 2? "In progress" : "Available"}</span>
          {mesa % 2 == 1 &&
            <>
              <span>details</span>
              <span>bill</span>
              <span>end</span>
            </>
          }
          {mesa % 2 == 0 &&
            <>
              <span>new session</span>
            </>
          }
        </div>
        <br />
      </div>
    ))}
  </>;
}

export default Tables;