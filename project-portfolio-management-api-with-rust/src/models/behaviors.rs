//! `ActiveModelBehavior` impls for the tables added after the first
//! capability waves (one line each; no custom hooks).

// One line per table; a glob keeps the generator's output trivially mergeable.
#![allow(clippy::wildcard_imports)]

use loco_rs::prelude::*;

use super::_entities::*;

impl ActiveModelBehavior for components::ActiveModel {}
impl ActiveModelBehavior for custom_field_defs::ActiveModel {}
impl ActiveModelBehavior for custom_field_values::ActiveModel {}
impl ActiveModelBehavior for watchers::ActiveModel {}
