use builder::IBuilder;
use coffee_machine::*;
use module::IModule;

fn main() {
    let mut builder = CoffeeMachine::create_builder();

    builder
        .add_module(|| print!("{}", "wow"))
        .add_module(|| print!("{}", "wowzah"))
        .add_module(|| print!("{}", "cooly"));

    let coffee_machine = builder.build();

    coffee_machine.run();
}

pub mod builder;
pub mod coffee_machine;
pub mod module;
