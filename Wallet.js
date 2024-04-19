import React from 'react';

function Wallet({ account, balance }) {
  return (
    <div className="Wallet">
      <h2>Wallet</h2>
      <p>Account: {account}</p>
      <p>Balance: {balance} ETH</p>
    </div>
  );
}

export default Wallet;
