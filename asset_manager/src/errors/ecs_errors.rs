use thiserror::Error;

#[derive(Debug, Error)]
pub enum EcsErrors {
  #[error("Attempting to add component to an entitity without calling create component first!")]
  CreateComponentNeverCalled,
  #[error("Attempted to use an unregisted component")]
  ComponentNotRegistered,
  #[error("Attempted to reference an entity that does not exist")]
  EntityDoesNotExist,
  #[error("Attempted to use component data that does not exist")]
  ComponentDataDoesNotExist,
  #[error("Attempted to downcast component to the wrong type")]
  DowncastToWrongType,
  #[error("No resource found at given path")]
  NoResourceAtPath,
  #[error("Unable to read the exe at the given path")]
  ExeResourceRegistrationFailed
}