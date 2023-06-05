import { useState } from 'react'
import Header from './components/header/Header';
import Landing from './pages/Landing';
import Tables from './pages/Menu/Tables';
import Details from './pages/Menu/Details';
import Bill from './pages/Menu/Bill';
import Sessions from './pages/Menu/Sessions';
import Session from './model/api/Session';

enum Menu {
  Tables,
  Details,
  Bill,
  Sessions
}

function App() {
  const [begin, setBegin] = useState(true);
  const [menu, setMenu] = useState(Menu.Tables);

  const [selected, setSelected] = useState<Session | null>(null);

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

  const loopMenu = () => { // TODO debug
    switch (menu) {
      case Menu.Tables:
        setMenu(Menu.Details);
        break;
      case Menu.Details:
        setMenu(Menu.Bill);
        break;
      case Menu.Bill:
        setMenu(Menu.Sessions);
        break;
      case Menu.Sessions:
        setMenu(Menu.Tables);
        break;
    }
  };

  const toSessions = () => {
    setSelected(null);
    setMenu(Menu.Sessions);
  }

  const toTables = () => {
    setSelected(null);
    setMenu(Menu.Tables);
  }

  const toDetails = (selected: Session) => {
    setSelected(selected);
    setMenu(Menu.Details);
  }

  const toBill = (selected: Session) => {
    setSelected(selected);
    setMenu(Menu.Bill);
  }

  const menuHtml = () => {
    switch (menu) {
      case Menu.Tables:
        return <Tables
          onDetails={toDetails}
          onBill={toBill}
        />;
      case Menu.Details:
        if (selected == null)
          throw new Error("selected is null");
        return <Details
          session={selected}
          onClose={toTables}
        />;
      case Menu.Bill:
        if (selected == null)
          throw new Error("selected is null");
        return <Bill />; // TODO
      case Menu.Sessions:
        return <Sessions 
          onClose={toTables}
        />;
    }
  };

  return (<>
    <Header
      onRefresh={ftRestart}
      options={[
        {
          label: "Sessions",
          onClick: toSessions,
        }
      ]}
    />
    <div style={{
      margin: "9px",
    }}>
      <button onClick={loopMenu}>Loop</button>
      {menuHtml()}
    </div>
  </>);
}

export default App
