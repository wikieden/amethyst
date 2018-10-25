//! Contains common types that can be glob-imported (`*`) for convenience.

pub use app::{Application, ApplicationBuilder, CoreApplication};
pub use config::Config;
pub use core::WithNamed;
pub use ecs::prelude::{Builder, World};
pub use game_data::{DataInit, GameData, GameDataBuilder};
pub use state::{
    EmptyState, EmptyTrans, SimpleState, SimpleTrans, State, StateData, Trans, TransQueue,
};
pub use state_event::StateEvent;
