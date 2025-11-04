fn main() {

    let mut user = User::new();
    let mut pickle_shop = get_pickle_shop_inventory();

    pickle_shop.display();
}

fn get_pickle_shop_inventory() -> Inventory {
    Inventory {
        inventory: vec![
        Item{name: "Cucumber".into(), visual: '🥒', price: 2, count: 5},
        Item{name: "Tomato".into(), visual: '🍅', price: 7, count: 3},
        Item{name: "Ginger".into(), visual: '🫚', price: 3, count: 3},
        Item{name: "Cabbage".into(), visual: '🥬', price: 4, count: 3},
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


impl Item {
    fn increase_count(&mut self) {
        self.count += 1;
    }
    
    fn decrease_count(&mut self) {
        if self.count > 0 {
            self.count -= 1;
        }
    }
    
    fn has_stock(&self) -> bool {
        self.count >= 1
    }

    fn display(&self) {
        println!("{} {} {} Pickle(s)", self.visual, self.count, self.name);
    }

    fn display_price(&self) {
        println!("{} {} Pickles cost ${} each", self.visual, self.name, self.price);
    }
}

#[derive(Debug)]
struct Inventory {
    inventory: Vec<Item>
}

impl Inventory {
    fn new() -> Inventory {
        Inventory {
            inventory: Vec::new()
        }
    }

    fn retrieve_item_index(&self, item_name: String) -> isize {
        
        for (index , item) in self.inventory.iter().enumerate() {
            if item.name == item_name {
                return index as isize
            }
        }
        -1
    }

    fn display(&self) {
        for item in &self.inventory {
            item.display();
        }
    }

    fn display_prices(&self) {
        for item in 
    }
}

struct User {
    cash: u32,
    inventory: Inventory,
}

impl User {
    fn new() -> Self {
        User {
            cash: 100,
            inventory: Inventory::new(),
        }
    }

    fn pay(&mut self, price: u32) {
        self.cash -= price;
    }

}
