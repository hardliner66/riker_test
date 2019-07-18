use std::time::Duration;

use interact_prompt::{LocalRegistry, Settings};
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

fn main() -> Result<(), interact_prompt::PromptError> {
    let sys = ActorSystem::new().unwrap();

    let props = Props::new_args(Box::new(|value| MyActor {
        value
    }), 0);

    let a_ref = sys
        .actor_of(props,
                  "actor_of",
        )
        .unwrap();

    let props = Props::new_args(Box::new(|value| MyActor {
        value
    }), 0);

    let b_ref = sys.
        actor_of(
            props,
            "actor_of_props",
        )
        .unwrap();


    let props = Props::new_args(Box::new(|value| MyActor {
        value
    }), 5);
    let c_ref = sys.
        actor_of(
            props,
            "actor_of_args",
        )
        .unwrap();

    LocalRegistry::insert("sys", Box::new(sys));

    a_ref.tell(
        s!("msg"),
        None,
    );

    b_ref.tell(
        s!("msg"),
        None,
    );

    c_ref.tell(
        s!("msg"),
        None,
    );

    interact_prompt::direct(Settings::default(), ())?;
    Ok(())
}


//#[macro_use]
//extern crate log;
//
//use std::time::Duration;
//
//use riker::actors::*;
//
//struct MyActor;
//
//// implement the Actor trait
//impl Actor for MyActor {
//    type Msg = String;
//
//    fn recv(&mut self,
//            _ctx: &Context<String>,
//            msg: String,
//            _sender: Sender) {
//        debug!("Received: {}", msg);
//    }
//}
//
//// provide factory and props functions
//impl MyActor {
//    fn actor() -> Self {
//        MyActor
//    }
//
//    fn props() -> BoxActorProd<MyActor> {
//        Props::new(Box::new(MyActor::actor))
//    }
//}
//
//// start the system and create an actor
//fn main() {
//    let sys = ActorSystem::new().unwrap();
//
//    let props = MyActor::props();
//    let my_actor = sys.actor_of(props, "my-actor").unwrap();
//
//    my_actor.tell("Hello my actor!".to_string(), None);
//
//    // force main to wait before exiting program
//    std::thread::sleep(Duration::from_millis(500));
//}


//use std::time::Duration;
//
//use interact_prompt::{LocalRegistry, Settings};
//use riker::actors::*;
//use stringlit::s;
//
//#[derive(Default)]
//struct MyActor {
//    value: i32,
//}
//
//// implement the Actor trait
//impl Actor for MyActor {
//    type Msg = String;
//
//    fn recv(&mut self,
//            ctx: &Context<String>,
//            msg: String,
//            _sender: Sender) {
//        println!("[{}] :: value: {} :: Received: {}", ctx.myself.name(), self.value, msg);
//    }
//}
//
//impl ActorFactoryArgs for MyActor {
//    type Args = i32;
//
//    fn create_args(args: Self::Args) -> BoxActorProd<Self> {
//        Props::new_args(Box::new(|value| MyActor {
//            value
//        }), args)
//    }
//}
//
//fn main() -> Result<(), interact_prompt::PromptError> {
//    let sys = ActorSystem::new().unwrap();
//
//    let a_ref = sys
//        .actor_of::<MyActor>(
//            "actor_of"
//        )
//        .unwrap();
//
//    let b_ref = sys.
//        actor_of_props(
//            MyActor::create(),
//            "actor_of_props",
//        )
//        .unwrap();
//
//    let c_ref = sys.
//        actor_of_args::<MyActor>(
//            "actor_of_args",
//            5,
//        )
//        .unwrap();
//
//    LocalRegistry::insert("sys", Box::new(sys));
//
//    a_ref.tell(
//        s!("msg"),
//        None,
//    );
//
//    b_ref.tell(
//        s!("msg"),
//        None,
//    );
//
//    c_ref.tell(
//        s!("msg"),
//        None,
//    );
//
//    interact_prompt::direct(Settings::default(), ())?;
//    Ok(())
//}
//
//
////#[macro_use]
////extern crate log;
////
////use std::time::Duration;
////
////use riker::actors::*;
////
////struct MyActor;
////
////// implement the Actor trait
////impl Actor for MyActor {
////    type Msg = String;
////
////    fn recv(&mut self,
////            _ctx: &Context<String>,
////            msg: String,
////            _sender: Sender) {
////        debug!("Received: {}", msg);
////    }
////}
////
////// provide factory and props functions
////impl MyActor {
////    fn actor() -> Self {
////        MyActor
////    }
////
////    fn props() -> BoxActorProd<MyActor> {
////        Props::new(Box::new(MyActor::actor))
////    }
////}
////
////// start the system and create an actor
////fn main() {
////    let sys = ActorSystem::new().unwrap();
////
////    let props = MyActor::props();
////    let my_actor = sys.actor_of(props, "my-actor").unwrap();
////
////    my_actor.tell("Hello my actor!".to_string(), None);
////
////    // force main to wait before exiting program
////    std::thread::sleep(Duration::from_millis(500));
////}
