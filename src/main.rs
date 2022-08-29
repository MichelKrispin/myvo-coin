mod transaction;

fn main() {
    let transaction = transaction::Transaction::empty();
    println!("{}", transaction);
}
