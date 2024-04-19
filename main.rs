// Contract to manage wallpapers
extern crate ink_prelude;
extern crate ink_storage;
extern crate ink_env;
extern crate ink_core;
extern crate ink_prelude;
extern crate ink_storage;


use ink_prelude::vec::Vec; // Import Vec from ink_prelude
use ink_storage::{collections::{HashMap as StorageHashMap, Vec as StorageVec}, traits::{PackedLayout, SpreadLayout}}; // Import HashMap, Vec, and storage traits from ink_storage
use ink_env::AccountId; // Import AccountId from ink_env
use ink_core::storage; // Import storage from ink_core

#![cfg_attr(not(feature = "std"), no_std)]



use ink_lang as ink;

#[ink::contract]
mod wallpaper_contract {
    use ink_prelude::vec::Vec;
    use ink_storage::{
        collections::{HashMap as StorageHashMap, Vec as StorageVec},
        traits::{PackedLayout, SpreadLayout},
    };

    #[ink(storage)]
    pub struct WallpaperContract {
        owner: AccountId,
        wallpapers: StorageVec<Wallpaper>,
        balances: StorageHashMap<AccountId, Balance>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct Wallpaper {
        id: u64,
        name: String,
        price: Balance,
        owner: AccountId,
    }

    #[ink(event)]
    pub struct Bought {
        #[ink(topic)]
        buyer: AccountId,
        #[ink(topic)]
        wallpaper_id: u64,
        #[ink(topic)]
        price: Balance,
        
    }

    impl WallpaperContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                wallpapers: StorageVec::new(),
                balances: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn buy_wallpaper(&mut self, id: u64) -> Result<(), &'static str> {
            let caller = self.env().caller();
            let mut wallpaper = self.wallpapers.get_mut(id).ok_or("Invalid wallpaper ID")?;
            let price = wallpaper.price;

            let buyer_balance = self.balance_of(caller);
            if buyer_balance < price {
                return Err("Insufficient balance");
            }

            let owner_balance = self.balance_of(wallpaper.owner);
            self.balances.insert(wallpaper.owner, owner_balance + price);
            self.balances.insert(caller, buyer_balance - price);
            wallpaper.owner = caller;

            self.env().emit_event(Bought {
                buyer: caller,
                wallpaper_id: id,
                price,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Balance {
            *self.balances.get(&account).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn add_wallpaper(&mut self, name: String, price: Balance) {
            self.wallpapers.push(Wallpaper {
                id: self.wallpapers.len() as u64,
                name,
                price,
                owner: self.env().caller(),
            });
        }

        #[ink(message)]
        pub fn get_wallpapers(&self) -> Vec<Wallpaper> {
            self.wallpapers.clone()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_buy_wallpaper() {
            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().unwrap();
            let mut contract = WallpaperContract::new();
            contract.add_wallpaper("Wallpaper 1".to_string(), 100);
            contract.add_wallpaper("Wallpaper 2".to_string(), 200);

            // Initial buyer balance
            assert_eq!(contract.balance_of(accounts.alice), 0);

            // Buy a wallpaper
            contract.buy_wallpaper(0).unwrap();

            // Verify buyer balance after purchase
            assert_eq!(contract.balance_of(accounts.alice), 100);
        }
    }
}
// #![cfg_attr(not(feature = "std"), no_std)]


// extern crate ink_prelude;
// extern crate ink_storage;
// extern crate ink_env;
// extern crate ink_core;

// use ink_prelude::{string::String, vec::Vec};
// use ink_storage::{
//     collections::{HashMap as StorageHashMap, Vec as StorageVec},
//     traits::{PackedLayout, SpreadLayout},
// };
// use ink_env::AccountId;
// use ink_lang as ink;

// type Balance = u64;

// #[ink::contract]
// mod wallpaper_contract {
//     use super::*;

//     #[ink(storage)]
//     pub struct WallpaperContract {
//         owner: AccountId,
//         wallpapers: StorageVec<Wallpaper>,
//         balances: StorageHashMap<AccountId, Balance>,
//     }

//     // Define types for Wallpapers
//     #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
//     pub struct Wallpaper {
//         id: u64,
//         name: String,
//         price: Balance,
//         owner: AccountId,
//         type AccountId = u64;
//         type Balance = u64;
//     }

//     // Events
//     #[ink(event)]
//     pub struct Bought {
//         #[ink(topic)]
//         buyer: AccountId,
//         #[ink(topic)]
//         wallpaper_id: u64,
//         #[ink(topic)]
//         price: Balance,
//     }

//     impl WallpaperContract {
//         #[ink(constructor)]
//         pub fn new() -> Self {
//             Self {
//                 owner: Self::env().caller(),
//                 wallpapers: StorageVec::new(),
//                 balances: StorageHashMap::new(),
//             }
//         }

//         #[ink(message)]
//         pub fn buy_wallpaper(&mut self, id: u64) -> Result<(), &'static str> {
//             let caller = self.env().caller();
//             let mut wallpaper = self.wallpapers.get_mut(id).ok_or("Invalid wallpaper ID")?;
//             let price = wallpaper.price;

//             let buyer_balance = self.balance_of(caller);
//             if buyer_balance < price {
//                 return Err("Insufficient balance");
//             }

//             let owner_balance = self.balance_of(wallpaper.owner);
//             self.balances.insert(wallpaper.owner, owner_balance + price);
//             self.balances.insert(caller, buyer_balance - price);
//             wallpaper.owner = caller;

//             self.env().emit_event(Bought {
//                 buyer: caller,
//                 wallpaper_id: id,
//                 price,
//             });

//             Ok(())
//         }

//         #[ink(message)]
//         pub fn balance_of(&self, account: AccountId) -> Balance {
//             *self.balances.get(&account).unwrap_or(&0)
//         }

//         #[ink(message)]
//         pub fn owner(&self) -> AccountId {
//             self.owner
//         }

//         #[ink(message)]
//         pub fn add_wallpaper(&mut self, name: String, price: Balance) {
//             self.wallpapers.push(Wallpaper {
//                 id: self.wallpapers.len() as u64,
//                 name,
//                 price,
//                 owner: self.env().caller(),
//             });
//         }

//         #[ink(message)]
//         pub fn get_wallpapers(&self) -> Vec<Wallpaper> {
//             self.wallpapers.clone()
//         }
//     }

//     #[cfg(test)]
//     mod tests {
//         use super::*;

//         #[test]
//         fn test_buy_wallpaper() {
//             let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().unwrap();
//             let mut contract = WallpaperContract::new();
//             contract.add_wallpaper("Wallpaper 1".to_string(), 100);
//             contract.add_wallpaper("Wallpaper 2".to_string(), 200);

//             // Initial buyer balance
//             assert_eq!(contract.balance_of(accounts.alice), 0);

//             // Buy a wallpaper
//             contract.buy_wallpaper(0).unwrap();

//             // Verify buyer balance after purchase
//             assert_eq!(contract.balance_of(accounts.alice), 100);
//         }
//     }
// }
