fn main() {
    let mut cash = 100;

    let mut shop_inventory = get_pickle_shop_inventory();

}

fn get_pickle_shop_inventory() -> ShopInventory {
    ShopInventory {
        inventory: vec![
        Item{name: "Pickling Brine".into(), visual: '🫙', price: 3, count: 20},
        Item{name: "Cucumber".into(), visual: '🥒', price: 2, count: 5},
        Item{name: "Tomato".into(), visual: '🍅', price: 7, count: 3},
        Item{name: "Ginger".into(), visual: '🫚', price: 3, count: 3},
        Item{name: "Napa Cabbage".into(), visual: '🥬', price: 4, count: 3},
        Item{name: "Watermelon".into(), visual: '🍉', price: 6, count: 3},
        Item{name: "Green Pepper".into(), visual: '🫑', price: 5, count: 3},
        Item{name: "Onion".into(), visual: '🧅', price: 3, count: 3},
        Item{name: "Eggplant".into(), visual: '🍆', price: 10, count: 3}, 
        Item{name: "Carrot".into(), visual: '🥕', price: 4, count: 3},
    ]
    }
}

#[derive(Debug)]
struct Item {
    name: String,
    visual: char,
    price: u32,
    count: u32,
}

#[derive(Debug)]
struct ShopInventory {
    inventory: Vec<Item>
}

impl ShopInventory {
    //fn buy_item()
    //fn display_item()
}

struct UserInventory {
    inventory: Vec<Item>
}

impl UserInventory {
    //fn add_item()
    //fn try_recipe(item1, item2) -> recipe_item
    // 
}