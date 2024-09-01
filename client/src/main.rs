use banklib::BankClient;

const SERVER_ADDRESS: &str = "127.0.0.1:7878";
const SERVER_ADDRESS2: &str = "127.0.0.1:7879";

fn main() {
    let bank_client = BankClient::new(SERVER_ADDRESS);
    let alice_account = bank_client.create_account("Alice".to_string());
    println!("{:?}", alice_account);

    let bob_account = bank_client.create_account("Bob".to_string());
    println!("{:?}", bob_account);

    let _ = bank_client.increase_account("Alice".to_string(), 10);
    let _ = bank_client.transfer("Alice".to_string(), "Bob".to_string(), 5);
    let _ = bank_client.decrease_account("Bob".to_string(), 2);

    let a = bank_client.get_account_balance("Alice".to_string()); //5
    println!("Alice balance = {:?}", a);
    let b = bank_client.get_account_balance("Bob".to_string()); //3
    println!("Bob balance = {:?}", b);

    let vec = bank_client.account_history("Alice".to_string());
    println!("Alice account operations history= {:?}", vec);

    let history = bank_client.get_history();
    println!("Bank operations history= {:?}", history);
    let history_len = history.len();

    let lib2 = BankClient::new(SERVER_ADDRESS2);
    lib2.restore(history);

    let history2_len = lib2.get_history();
    println!(
        "history_size = {:?} and new history_size = {:?}",
        history_len,
        history2_len.len()
    );
}
