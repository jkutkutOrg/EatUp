import { SetStateAction, useState } from 'react';
import Header from './components/header/Header';
import ServicesHandler from './components/serviceHandler/ServiceHandler';
import useWebsocket from './model/useWebsocket';

const App = () => {
  const socket = useWebsocket("ws://localhost:9000/ws");

  const refresh = () => {
    socket.send("/microservices");
  };

  return <>
    <Header
      onRefresh={refresh}
    />
    <ServicesHandler
      services={socket.message}
    />
  </>
}

export default App;
