import React, { useState, useEffect } from 'react';
import Wallpapers from './Wallpapers';
import Wallet from './Wallet';
import { connectWallet } from './web3';

function App() {
  const [account, setAccount] = useState('');
  const [balance, setBalance] = useState(0);

  useEffect(() => {
    async function fetchAccount() {
      const { account, balance } = await connectWallet();
      setAccount(account);
      setBalance(balance);
    }
    fetchAccount();
  }, []);

  return (
    <div className="App">
      <h1>Wallpaper Marketplace</h1>
      <Wallet account={account} balance={balance} />
      <Wallpapers account={account} />
    </div>
  );
}

export default App;
