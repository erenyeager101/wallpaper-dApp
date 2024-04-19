import React, { useState, useEffect } from 'react';
import { buyWallpaper, getWallpapers } from './web3';

function Wallpapers({ account }) {
  const [wallpapers, setWallpapers] = useState([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    async function fetchWallpapers() {
      setLoading(true);
      const data = await getWallpapers();
      setWallpapers(data);
      setLoading(false);
    }
    fetchWallpapers();
  }, []);

  const handleBuy = async (id, price) => {
    try {
      await buyWallpaper(id);
      alert('Wallpaper bought successfully!');
    } catch (error) {
      alert('Failed to buy wallpaper');
    }
  };

  return (
    <div className="Wallpapers">
      <h2>Wallpapers</h2>
      {loading ? (
        <p>Loading...</p>
      ) : (
        <div>
          {wallpapers.map((wallpaper) => (
            <div key={wallpaper.id} className="wallpaper">
              <img src={`images/${wallpaper.id}.jpg`} alt={wallpaper.name} />
              <p>{wallpaper.name}</p>
              <p>Price: {wallpaper.price} ETH</p>
              {account && (
                <button onClick={() => handleBuy(wallpaper.id, wallpaper.price)}>
                  Buy
                </button>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

export default Wallpapers;
