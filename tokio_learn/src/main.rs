use tokio::io::{self, AsyncWriteExt};

async fn run() {
	let mut cout = io::stdout();
	_ = cout.write_all(b"hello, Async!\n").await;
	_ = cout.flush().await;
}



#[tokio::main]
async fn main() {
	_ = run().await;
}
