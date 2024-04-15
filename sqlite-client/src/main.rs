mod client;
mod errors;
mod db;
fn main() {
    // Example sharing one connection between threads with lock
    let sqlite_client = client::MtSqLiteClient::new_in_memory().unwrap();
    if let con = sqlite_client.connection.lock().unwrap() {
        match db::person::PersonRepository::create_table(&con) {
            Ok(_) => println!("Tabled created"),
            Err(_) => println!("Error creating table")
        };
    };
    let con = sqlite_client.connection.clone();
    let handler = std::thread::spawn(move || {
        if let con = con.lock().unwrap() {
            match db::car::CarRepository::create_table(&con) {
                Ok(_) => println!("Tabled created"),
                Err(_) => println!("Error creating table")
            };
        };
    });

    handler.join().unwrap();
}
