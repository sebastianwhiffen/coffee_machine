use crate::{builder::IBuilder, module::IModule};

pub struct CoffeeMachine {
    modules: Vec<fn() -> ()>,
}

pub struct CoffeeMachineBuilder {
    pub modules: Vec<fn() -> ()>,
}

impl IBuilder<CoffeeMachine> for CoffeeMachineBuilder {
    fn build(&self) -> CoffeeMachine {
        CoffeeMachine {
            modules: self.modules.clone(),
        }
    }

    fn add_module(&mut self, module: fn() -> ()) -> &mut Self {
        self.modules.push(module);
        self
    }
}

impl IModule<CoffeeMachine, CoffeeMachineBuilder> for CoffeeMachine {
    fn run(&self) {
        for module in &self.modules {
            module();
        }
    }

    fn create_builder() -> CoffeeMachineBuilder {
        CoffeeMachineBuilder {
            modules: Vec::new(),
        }
    }
}
