//! Canvas model helpers (LT-1): the `ActiveModelBehavior` impls.

use loco_rs::prelude::*;

use super::_entities::{canvas_items, canvases};

impl ActiveModelBehavior for canvases::ActiveModel {}
impl ActiveModelBehavior for canvas_items::ActiveModel {}
