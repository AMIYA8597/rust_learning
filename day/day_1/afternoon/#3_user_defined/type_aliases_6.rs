// A type alias creates a name for another type. The two types can be used interchangeably.





// enum carryItem {
//     Left,
//     Right,
// }

// type items = carryItem;

// // Aliases are more useful with long, complex types:

// use std::cell::RefCell;
// use std::sync::{Arc, RwLock};
// type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;






















// Define an enum for carrying items
enum carryItem {
    Left,
    Right,
}

// Create a type alias for carryItem
type items = carryItem;

// Import necessary modules from the standard library
use std::cell::RefCell;
use std::sync::{Arc, RwLock};

// Define a struct for an Item
struct Item {
    name: String,
    weight: f32,
}

// Create a type alias for PlayerInventory
type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;

// Function to add an item to the player inventory
fn add_item(inventory: &PlayerInventory, item: Arc<RefCell<Item>>) {
    let mut inv = inventory.write().unwrap();
    inv.push(item);
}

// Function to list all items in the player inventory
fn list_items(inventory: &PlayerInventory) {
    let inv = inventory.read().unwrap();
    for item in inv.iter() {
        let item = item.borrow();
        println!("Item: {}, Weight: {}", item.name, item.weight);
    }
}

// Main function
fn main() {
    // Create a new PlayerInventory
    let player_inventory: PlayerInventory = RwLock::new(Vec::new());

    // Create some items
    let item1 = Arc::new(RefCell::new(Item { name: String::from("Sword"), weight: 3.5 }));
    let item2 = Arc::new(RefCell::new(Item { name: String::from("Shield"), weight: 5.0 }));

    // Add items to the inventory
    add_item(&player_inventory, item1.clone());
    add_item(&player_inventory, item2.clone());

    // List items in the inventory
    println!("Player Inventory:");
    list_items(&player_inventory);
}







// enum CarryableConcreteItem {
//     Left,
//     Right,
// }

// type Item = CarryableConcreteItem;

// // Aliases are more useful with long, complex types:
// use std::cell::RefCell;
// use std::sync::{Arc, RwLock};
// type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;