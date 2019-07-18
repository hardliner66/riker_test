use std::time::Duration;

use riker::actors::*;
use stringlit::s;

#[derive(Default)]
struct MyActor {
    value: i32,
}

// implement the Actor trait
impl Actor for MyActor {
    type Msg = String;

    fn recv(&mut self,
            ctx: &Context<String>,
            msg: String,
            _sender: Sender) {
        println!("[{}] :: value: {} :: Received: {}", ctx.myself.name(), self.value, msg);
    }
}

impl ActorFactoryArgs for MyActor {
    type Args = i32;

    fn create_args(args: Self::Args) -> BoxActorProd<Self> {
        Props::new_args(Box::new(|value| MyActor {
            value
        }), args)
    }
}

fn main() {
    let sys = ActorSystem::new().unwrap();

    let a_ref = sys
        .actor_of::<MyActor>(
            "actor_of"
        )
        .unwrap();

    a_ref.tell(
        s!("msg"),
        None,
    );

    let b_ref = sys.
        actor_of_props(
            MyActor::create(),
            "actor_of_props",
        )
        .unwrap();

    b_ref.tell(
        s!("msg"),
        None,
    );

    let c_ref = sys.
        actor_of_args::<MyActor>(
            "actor_of_args",
            5,
        )
        .unwrap();

    c_ref.tell(
        s!("msg"),
        None,
    );

    // force main to wait before exiting program
    std::thread::sleep(Duration::from_millis(500));
}
