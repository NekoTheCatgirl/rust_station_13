
/// Define machine components. Each component gives stats (to be handled by the work_tick)
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Component {

}

/// Define the basic data a machine uses, all should have getters and setters (no publics)
pub struct Machine {
    components: Vec<(Component, u16)>,
}

impl Machine {
    pub fn get_components(&self) -> Vec<(Component, u16)> {
        return self.components.clone();
    }
}

/// A trait required for all types that work with Machine.
pub trait MachineInterface {
    fn work_tick(&mut self, &mut Machine);
    fn required_components(&self) -> Vec<(Component, u16)>;
    fn installed_components(&self, &Machine) -> Vec<(Component, u16)>;
}
