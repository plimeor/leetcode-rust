use generator::create_problem;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please input problem id");
        return;
    }

    create_problem(args[1].trim().to_string()).await.unwrap();
}
