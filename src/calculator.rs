use unity::{prelude::OptionalMethod, system::Il2CppString};

#[unity::class("App", "GameCalculatorCommand")]
pub struct GameCalculatorCommand {
    pub parent: CalculatorCommand,
}

impl GameCalculatorCommand {
    pub fn reverse(&self) -> &'static mut GameCalculatorCommand {
        unsafe { game_calculator_command_reverse(self, None) }
    }
}

#[unity::class("App", "CalculatorCommand")]
pub struct CalculatorCommand {}

#[unity::class("App", "CalculatorManager")]
pub struct CalculatorManager {}

impl CalculatorManager {
    pub fn add_command(&mut self, command: &CalculatorCommand) -> &'static mut CalculatorCommand {
        unsafe { calculator_manager_add_command(self, command, None) }
    }

    pub fn find_command<'a>(&mut self, name: impl Into<&'a Il2CppString>) -> &'static mut CalculatorCommand {
        unsafe { calculator_manager_find_command(self, name.into(), None) }
    }
}

#[unity::from_offset("App", "CalculatorManager", "AddCommand")]
fn calculator_manager_add_command(this: &mut CalculatorManager, command: &CalculatorCommand, method_info: OptionalMethod) -> &'static mut CalculatorCommand;

#[skyline::from_offset(0x0298daa0)]
fn calculator_manager_find_command(this: &CalculatorManager, name: &Il2CppString, method_info: OptionalMethod) -> &'static mut CalculatorCommand;

#[unity::from_offset("App", "GameCalculatorCommand", "Reverse")]
fn game_calculator_command_reverse(this: &GameCalculatorCommand, method_info: OptionalMethod) -> &'static mut GameCalculatorCommand;