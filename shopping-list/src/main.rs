use shopping_list::ShoppingList;

fn main() {
    let mut shopping_list = match ShoppingList::from("test.txt") {
        Ok(sl) => sl,
        Err(_) => std::process::exit(1),
    };

    shopping_list.update("tomato", 10);
    shopping_list.insert("potato", 20);
    shopping_list.insert("blueberry", 40);

    shopping_list.dedup();
    shopping_list.sort();

    println!("{}", shopping_list);

    shopping_list.save().expect("Failed to save to file");
}
