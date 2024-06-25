use actix::{Actor, Context, Handler, Message};

struct FizzBuzz;

impl Actor for FizzBuzz {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "String")]
struct FizzBuzzRequest(usize);

impl Handler<FizzBuzzRequest> for FizzBuzz {
    type Result = String;

    fn handle(&mut self, _msg: FizzBuzzRequest, _ctx: &mut Context<Self>) -> Self::Result {
        "x".to_string()
    }
}

#[actix::main]
async fn main() {

    let addr = FizzBuzz.start();

    let result = addr.send(FizzBuzzRequest(1)).await;

    println!("RESULT: {}", result.unwrap());

    println!("Hello, world!");
}
