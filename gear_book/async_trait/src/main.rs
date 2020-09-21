use async_trait::async_trait;

#[async_trait]
trait AsyncTrait {
    async fn f(&self);
}

struct Runner {}

#[async_trait]
impl AsyncTrait for Runner {
    async fn f(&self) {
        println!("Hello, async-trait(impl side)")
    }
}

#[async_std::main]
async fn main() {
    let runner = Runner {};
    runner.f().await;
}
