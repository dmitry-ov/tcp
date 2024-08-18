use banklib::Lib;

const SERVER_ADDRESS: &str = "127.0.0.1:7878";
const SERVER_ADDRESS2: &str = "127.0.0.1:7879";

fn main() {
    let lib = Lib::new(SERVER_ADDRESS);
    let alice_account = lib.create_account("Alice".to_string());
    println!("{:?}", alice_account);

    let bob_account = lib.create_account("Bob".to_string());
    println!("{:?}", bob_account);

    let _ = lib.increase_account("Alice".to_string(), 10);
    let _ = lib.transfer("Alice".to_string(), "Bob".to_string(), 5);
    let _ = lib.decrease_account("Bob".to_string(), 2);

    let a = lib.get_account_balance("Alice".to_string()); //5
    println!("Alice balance = {:?}", a);
    let b = lib.get_account_balance("Bob".to_string()); //3
    println!("Bob balance = {:?}", b);

    let vec = lib.account_history("Alice".to_string());
    println!("Alice account operations history= {:?}", vec);

    let history = lib.get_history();
    println!("Bank operations history= {:?}", history);
    let history_len = history.len();

    let lib2 = Lib::new(SERVER_ADDRESS2);
    let _ = lib2.restore(history);

    let history2_len = lib2.get_history();
    println!("history_size = {:?} and new history_size = {:?}", history_len, history2_len.len());
}
