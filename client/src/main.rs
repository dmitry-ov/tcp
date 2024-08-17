use banklib::Lib;

fn main() {
    // let mut lib = Lib::new();
    let alice_account = Lib::create_account("Alice".to_string());
    println!("{:?}", alice_account);

    let bob_account = Lib::create_account("Bob".to_string());
    println!("{:?}", bob_account);

    let _ = Lib::increase_account("Alice".to_string(), 10);
    let _ = Lib::transfer("Alice".to_string(), "Bob".to_string(), 5);
    let _ = Lib::decrease_account("Bob".to_string(), 2);

    let a = Lib::get_account_balance("Alice".to_string()); //5
    println!("Alice balance = {:?}", a);
    let b = Lib::get_account_balance("Bob".to_string()); //3
    println!("Bob balance = {:?}", b);

    let history = Lib::get_history();
    println!("Bank operations history= {:?}", history);

    let vec = Lib::account_history("Alice".to_string());
    println!("Alice account operations history= {:?}", vec);
}
