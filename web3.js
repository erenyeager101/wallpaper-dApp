// Import web3 library (you may need to install it using npm or yarn)
import Web3 from 'web3';

// Replace with your smart contract ABI and address
const contractABI = [
  // ABI of your smart contract
];

const contractAddress = '0x...'; // Address of your deployed smart contract

// Initialize Web3
const web3 = new Web3(window.ethereum);

// Function to connect wallet
export async function connectWallet() {
  try {
    // Request account access
    await window.ethereum.request({ method: 'eth_requestAccounts' });
    // Get account address
    const accounts = await web3.eth.getAccounts();
    const account = accounts[0];
    // Get account balance
    const balance = await web3.eth.getBalance(account);
    return { account, balance: web3.utils.fromWei(balance, 'ether') };
  } catch (error) {
    throw new Error('Failed to connect wallet');
  }
}

// Function to get wallpapers
export async function getWallpapers() {
  try {
    const contract = new web3.eth.Contract(contractABI, contractAddress);
    const wallpapers = await contract.methods.get_wallpapers().call();
    return wallpapers;
  } catch (error) {
    throw new Error('Failed to fetch wallpapers');
  }
}

// Function to buy wallpaper
export async function buyWallpaper(id) {
  try {
    const contract = new web3.eth.Contract(contractABI, contractAddress);
    await contract.methods.buy_wallpaper(id).send({ from: window.ethereum.selectedAddress });
  } catch (error) {
    throw new Error('Failed to buy wallpaper');
  }
}
