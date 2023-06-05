import { useState } from 'react'
import Header from './components/header/Header';
import Landing from './pages/Landing';
import Tables from './pages/Menu/Tables';
import Details from './pages/Menu/Details';
import Bill from './pages/Menu/Bill';
import Sessions from './pages/Menu/Sessions';
import Session from './model/api/Session';
import Menu from './model/App/Menu';

function App() {
  const [menu, setMenu] = useState(Menu.Landing);
  const [selected, setSelected] = useState<Session | null>(null);

  const ftBegin = () => setMenu(Menu.Tables);
  const ftRestart = () => setMenu(Menu.Landing);

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
      case Menu.Landing:
        return <Landing
          onBegin={ftBegin}
        />;
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
        />;
      case Menu.Bill:
        if (selected == null)
          throw new Error("selected is null");
        return <Bill
          session={selected}
        />;
      case Menu.Sessions:
        return <Sessions 
          onBill={toBill}
        />;
    }
  };

  const headerOptions = [];
  if (menu && menu != Menu.Sessions) {
    headerOptions.push({
      label: "Sessions",
      onClick: toSessions,
    });
  }

  return (<>
    <Header
      onRefresh={ftRestart}
      onClose={toTables}
      menu={menu}
      extraOptions={headerOptions}
    />
    <div style={{
      margin: "9px",
    }}>
      {menuHtml()}
    </div>
  </>);
}

export default App
