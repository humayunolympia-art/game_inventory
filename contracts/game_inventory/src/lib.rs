#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map, String, Vec};

#[contract]
pub struct GameInventory;

#[contractimpl]
impl GameInventory {
    /// Initialize the contract
    pub fn init(env: Env) {
        if env.storage().instance().get::<_, bool>(&"initialized").is_some() {
            panic!("Already initialized");
        }
        env.storage().instance().set(&"initialized", &true);
    }

    /// Admin mints a new item and assigns it to an owner
    pub fn mint_item(env: Env, admin: Address, owner: Address, item_id: u64, item_type: String, stats: u64) {
        admin.require_auth();

        // Check if item already exists
        let items: Map<u64, (Address, String, u64)> = env
            .storage()
            .instance()
            .get(&"items")
            .unwrap_or(Map::new(&env));

        if items.contains_key(item_id) {
            panic!("Item already exists");
        }

        // Store the item: Map<item_id, (owner, item_type, stats)>
        let item_data = (owner.clone(), item_type, stats);
        let mut updated_items = items;
        updated_items.set(item_id, item_data);
        env.storage().instance().set(&"items", &updated_items);

        // Add item_id to user's inventory: Map<Address, Vec<u64>>
        let mut user_inventories: Map<Address, Vec<u64>> = env
            .storage()
            .instance()
            .get(&"user_inventories")
            .unwrap_or(Map::new(&env));

        let mut user_items = user_inventories.get(owner.clone()).unwrap_or(Vec::new(&env));
        user_items.push_back(item_id);
        user_inventories.set(owner, user_items);
        env.storage().instance().set(&"user_inventories", &user_inventories);
    }

    /// Transfer an item from one user to another
    pub fn transfer_item(env: Env, from: Address, to: Address, item_id: u64) {
        from.require_auth();

        // Get items map
        let mut items: Map<u64, (Address, String, u64)> = env
            .storage()
            .instance()
            .get(&"items")
            .unwrap_or(Map::new(&env));

        // Verify the caller owns the item
        let item_data = items.get(item_id).unwrap_or_else(|| panic!("Item not found"));
        let (current_owner, item_type, stats) = item_data;

        if current_owner != from {
            panic!("Not the owner of this item");
        }

        // Update the item owner
        items.set(item_id, (to.clone(), item_type, stats));
        env.storage().instance().set(&"items", &items);

        // Update user inventories
        let mut user_inventories: Map<Address, Vec<u64>> = env
            .storage()
            .instance()
            .get(&"user_inventories")
            .unwrap_or(Map::new(&env));

        // Remove item from sender's inventory
        let from_items = user_inventories.get(from.clone()).unwrap_or(Vec::new(&env));
        let mut new_from_items = Vec::new(&env);
        for i in 0..from_items.len() {
            let id = from_items.get(i).unwrap(); // Vec::get returns Option<T>
            if id != item_id {
                new_from_items.push_back(id);
            }
        }
        user_inventories.set(from, new_from_items);

        // Add item to receiver's inventory
        let mut to_items = user_inventories.get(to.clone()).unwrap_or(Vec::new(&env));
        to_items.push_back(item_id);
        user_inventories.set(to, to_items);
        env.storage().instance().set(&"user_inventories", &user_inventories);
    }

    /// Get the owner of an item
    pub fn get_owner(env: Env, item_id: u64) -> Address {
        let items: Map<u64, (Address, String, u64)> = env
            .storage()
            .instance()
            .get(&"items")
            .unwrap_or(Map::new(&env));

        let item_data = items.get(item_id).unwrap_or_else(|| panic!("Item not found"));
        item_data.0
    }

    /// Get item details (item_type, stats)
    pub fn get_item(env: Env, item_id: u64) -> (String, u64) {
        let items: Map<u64, (Address, String, u64)> = env
            .storage()
            .instance()
            .get(&"items")
            .unwrap_or(Map::new(&env));

        let item_data = items.get(item_id).unwrap_or_else(|| panic!("Item not found"));
        (item_data.1, item_data.2)
    }

    /// Get all item IDs owned by a user
    pub fn get_user_items(env: Env, user: Address) -> Vec<u64> {
        let user_inventories: Map<Address, Vec<u64>> = env
            .storage()
            .instance()
            .get(&"user_inventories")
            .unwrap_or(Map::new(&env));

        user_inventories.get(user).unwrap_or(Vec::new(&env))
    }
}
