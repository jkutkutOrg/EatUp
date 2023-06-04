import { useState } from 'react'
import Header from './components/header/Header';
import Landing from './pages/Landing';
import Tables from './pages/Menu/Tables';
import Details from './pages/Menu/Details';
import Bill from './pages/Menu/Bill';
import Sessions from './pages/Menu/Sessions';

enum Menu {
  Tables,
  Details,
  Bill,
  Sessions
}

function App() {
  const [begin, setBegin] = useState(true);
  const [menu, setMenu] = useState(Menu.Tables);

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
    setMenu(Menu.Sessions);
  }

  const menuHtml = () => {
    switch (menu) {
      case Menu.Tables:
        return <Tables />;
      case Menu.Details:
        return <Details />;
      case Menu.Bill:
        return <Bill />;
      case Menu.Sessions:
        return <Sessions />;
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
