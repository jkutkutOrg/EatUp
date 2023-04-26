import { SetStateAction, useState } from 'react';
import Header from './components/header/Header';
import ServicesHandler from './components/serviceHandler/ServiceHandler';
import useWebsocket from './model/useWebsocket';

const App = () => {
  const socket = useWebsocket("ws://localhost:9000/ws");
  const [inputValue, setInputValue] = useState('');

  const handleInputChange = (event: { target: { value: SetStateAction<string>; }; }) => {
    setInputValue(event.target.value);
  };

  const handleButtonClick = () => {
    socket.send(inputValue);
    setInputValue('');
  };

  return <>
    <Header/>
    <ServicesHandler/>
    <br />
    <br />
    <div>
      <input
        type="text"
        value={inputValue}
        onChange={handleInputChange}
      />
      <button onClick={handleButtonClick}>Send</button>
      <div>
        <h5>Received Message:</h5>
        <p>{socket.message}</p>
      </div>
    </div>
  </>
}

export default App;
