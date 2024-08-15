
/// Define machine components. Each component gives stats (to be handled by the work_tick)
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Component {
    
}

/// Define the basic data a machine uses, all should have getters and setters (no publics)
pub struct Machine {
    panel: bool, // Dictates if the maint panel is open
    components: Vec<(Component, u16)>,
    required_components: Vec<(Component, u16)>,
}

impl Machine {
    /// Returns a vector of components and how many are inserted.
    pub fn get_components(&self) -> Vec<(Component, u16)> {
        self.components.clone()
    }

    /// Used to inspect if the maintinance panel is open for the machine.
    pub fn panel_open(&self) -> bool {
        self.panel
    }

    pub fn remove_component(&mut self, component: Component) -> Result<&str, &str> {
        
        todo!();
    }

    pub fn insert_component(&mut self, component: Component) -> Result<&str, &str> {
        

        todo!();
    }
}

/// A trait required for all types that work with Machine.
pub trait MachineInterface {
    fn work_tick(&mut self, machine: &mut Machine);
    fn required_components(&self) -> Vec<(Component, u16)>;
    fn installed_components(&self, machine: &Machine) -> Vec<(Component, u16)>;
}
