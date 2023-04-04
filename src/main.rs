use std::env;
#[async_std::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    yadbmt::run(args).await;
}
