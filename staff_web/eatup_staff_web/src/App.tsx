import { useState } from 'react'
import Header from './components/header/Header';
import Landing from './pages/Landing';

function App() {
  const [begin, setBegin] = useState(true);

  const ftBegin = () => {
    setBegin(false);
  };

  const ftRestart = () => {
    setBegin(true);
  };

  if (begin) {
    return (
      <Landing
        onHeaderRefresh={ftRestart}
        onBegin={ftBegin}
      />
    );
  }


  let mesas = [10, 11, 12, 13]

  return (<>
    <Header
      onRefresh={ftRestart}
      options={[
        {
          label: "Sessions",
          onClick: ftRestart, // TODO
        }
      ]}
    />
    <div style={{
      margin: "9px",
    }}>
      <div style={{
        display: "flex",
        flexDirection: "row",
        justifyContent: "space-between",
        alignItems: "center",
      }}>
        <h5>Logo</h5>
        <h5>Sessions</h5>
      </div>
      <h1>Begin</h1>

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

      <br />
      <br />
      <br />

      <div style={{
        display: "flex",
        flexDirection: "row",
        justifyContent: "space-between",
        alignItems: "center",
      }}>
        <h5>Logo</h5>
        <h5>X (close)</h5>
      </div>
      <h1>Details</h1>
      <h5>Mesa XX</h5>
      <h5>Id: XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX</h5>
      <h5>Simple_id: XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX</h5>
      <img src="" alt="QR code" />


      <br />
      <br />
      <br />

      <div style={{
        display: "flex",
        flexDirection: "row",
        justifyContent: "space-between",
        alignItems: "center",
      }}>
        <h5>Logo</h5>
        <h5>X (close)</h5>
      </div>
      <h1>Bill</h1>
      <h5>Session Id:</h5>
      <div style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-between",
        }}>
          <span>product name</span>
          <span>product.price</span>
          <span>product.quantity</span>
        </div>
      {[
        {
          name: "Item 1",
          price: 10,
          quantity: 2,
        },
        {
          name: "Item 2",
          price: 10,
          quantity: 2,
        },
        {
          name: "Item 3",
          price: 10,
          quantity: 2,
        },
        {
          name: "Item 4",
          price: 10,
          quantity: 2,
        }
      ].map((product) => {
        return <div key={product.name} style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-between",
        }}>
          <span>{product.name}</span>
          <span>{product.price}</span>
          <span>{product.quantity}</span>
        </div>
      })}
      <br />
      <span>Total: 32um</span>
    </div>
  </>);
}

export default App
