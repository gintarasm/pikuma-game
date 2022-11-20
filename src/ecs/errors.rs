use thiserror::Error;

use super::components::Component;


#[derive(Error, Debug)]
pub enum EcsErrors {

    #[error("Entity {0} does not exist")]
    EntityDoesNotExist(usize),
    
    #[error("Component {0} does not exist")]
    ComponentDoesNotExist(String)
}

impl EcsErrors {
    pub fn component_does_not_exist<T: Component + 'static>() -> Self {
        let name = std::any::type_name::<T>();
        Self::ComponentDoesNotExist(name.to_owned())
    }
}
