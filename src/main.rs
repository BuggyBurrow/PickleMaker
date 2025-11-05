use std::fmt;

fn main() {
    let mut user = User::new(String::from("Buggy"));
    let mut shop = Shop::new(String::from("Pickle Shop"));
    
    let cucumber = Item::new(String::from("Cucumber"), '🥒', 3);
    println!("{:?}", cucumber);

    let mut cucumber_stack = ItemStack::new(cucumber);
    cucumber_stack.add_one();
    cucumber_stack.add_one();
    cucumber_stack.subtract_one();

    let mut inventory = Inventory::new();
    inventory.0.push(cucumber_stack);
    


    
    println!("{:?}", inventory);


}

/*
fn get_pickle_shop_inventory() -> Inventory {
    Inventory {
        inventory: vec![
        Item{name: "Cucumber".into(), emoji: '🥒', price: 2, stock: 5},
        Item{name: "Tomato".into(), emoji: '🍅', price: 7, stock: 3},
        Item{name: "Ginger".into(), emoji: '🫚', price: 3, stock: 3},
        Item{name: "Cabbage".into(), emoji: '🥬', price: 4, stock: 3},
        Item{name: "Watermelon".into(), emoji: '🍉', price: 6, stock: 3},
        Item{name: "Green Pepper".into(), emoji: '🫑', price: 5, stock: 3},
        Item{name: "Onion".into(), emoji: '🧅', price: 3, stock: 3},
        Item{name: "Eggplant".into(), emoji: '🍆', price: 10, stock: 3}, 
        Item{name: "Carrot".into(), emoji: '🥕', price: 4, stock: 3},
    ]
    }
}
    */

#[derive(Debug)]
#[derive(Clone)]
struct Item {
    name: String,
    emoji: char,
    price: u32,
}

impl Item {
    fn new(name: String, emoji: char, price: u32) -> Self {
        Self {
            name,
            emoji,
            price, 
        }
    }
}

#[derive(Debug)]
struct ItemStack {
    item: Item,
    stack: u32,
}

impl ItemStack {
    fn new(item: Item) -> ItemStack {
        Self {
            item: item,
            stack: 1,
        }
    }

    fn add_one(&mut self) {
        self.stack += 1;
    }

    fn subtract_one(&mut self) -> Result<(),()> {
        if self.stack <= 0 {
            Err(())
        }
        else {
            self.stack -= 1;
            Ok(())
        }
    }
}

#[derive(Debug)]
struct Inventory(Vec<ItemStack>);

impl Inventory {
    fn new() -> Inventory {
        Inventory(Vec::new())
    }

    fn retrieve_item_index(&self, item_name: String) -> isize {
        for (index , item_stack) in self.0.iter().enumerate() {
            if item_stack.item.name == item_name {
                return index as isize
            }
        }
        -1
    }

    //fn add_item()
    //fn add_stack()
}

struct Shop {
    name: String,
    inventory: Inventory,
}

impl Shop {
    fn new(name: String) -> Shop {
        Shop {
            name,
            inventory: Inventory::new(),
        }
    }
}

struct User {
    name: String,
    cash: u32,
    inventory: Inventory,
}

impl User {
    fn new(name: String) -> Self {
        User {
            name,
            cash: 100,
            inventory: Inventory::new(),
        }
    }
/* 
fn add_item(&self, &item: Item) {
    let item_index = self.inventory.retrieve_item_index(item.name);
    if item_index < 0 {
        //self.inventory.0.push(ItemStack)
    }
}
*/

    fn pay(&mut self, price: u32) {
        self.cash -= price;
    }

    fn can_pay(&self, price: u32) -> bool {
        self.cash >= price
    }

    fn display_cash(&self) {
        println!("You have ${}", self.cash)
    }

}
