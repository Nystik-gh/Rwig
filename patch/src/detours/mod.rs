mod audio_client_detours;
mod detours;
mod render_client_detours;
mod vtable;

use audio_client_detours::*;
pub use detours::*;
use render_client_detours::*;
use vtable::*;
