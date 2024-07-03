use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Hello before reading file!");
    
    let h1 = tokio::spawn(async {
        let _file1_contents = read_from_file1().await;
        println!("{:?}", _file1_contents);
    });

    let h2 = tokio::spawn(async {
        let _file2_contents = read_from_file2().await;
        println!("{:?}", _file2_contents);
    });
    
    let _ = tokio::join!(h1, h2);
}

async fn read_from_file1() -> String {
    sleep(Duration::new(4, 0));
    println!("{:?}", "Processing file 1");
    String::from("Hello, there from file 1")
}

async fn read_from_file2() -> String {
    sleep(Duration::new(2, 0));
    println!("{:?}", "Processing file 2");
    String::from("Hello, there from file 2")
}
