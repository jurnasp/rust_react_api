import React from 'react';
import logo from './logo.svg';
import './App.css';
import {AppService} from './services/app.service';
import {useState, useEffect} from 'react';

function App() {

  const [user, setUser] = useState<any>();
  const appService = new AppService();

  const fetchUser = async() => {
    const user = await appService.getUser();
    setUser(user);
  };

  useEffect(() => {
    fetchUser();
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
      <div>{JSON.stringify(user)}</div>
    </div>
  );
}

export default App;
