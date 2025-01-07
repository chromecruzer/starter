use crossbeam::channel;
use datatypes::criteria::{Criteria, Gender};
use tokio::time::{sleep, Duration};
//use reqwest::blocking::Client;   // you cannot use tokio and reqwest blocking at same time

#[tokio::main]
async fn main() {
    println!("Starting...");
    sleep(Duration::from_secs(2)).await;
    println!("Finished after 2 seconds!");
    let mut person1 = Criteria::new(65, Gender::Others, "Indian".to_string());
    person1.display_stats();
    person1.mutable_opt(98);

    // let client = Client::new();
    // let response = client
    //     .get("https://jsonplaceholder.typicode.com/todos/1")
    //     .send();

    // let body = response.expect("Failed To Fetch").text().unwrap();

    // println!("Body: {}", body);

    ///// async Runtime
    println!("Hi");
    read_from_db(1).await;
    println!("Bye");
    read_from_db(2).await;

    let (sender, receiver) = channel::unbounded();

    std::thread::spawn(move || {
        sender.send("hello").unwrap();
    });

    println!("Received: {}", receiver.recv().unwrap());
}

async fn read_from_db(count: i8) {
    println!("Reading Success {}", count)
}
