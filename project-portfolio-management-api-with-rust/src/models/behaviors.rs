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
impl ActiveModelBehavior for approval_requests::ActiveModel {}
impl ActiveModelBehavior for task_relations::ActiveModel {}
impl ActiveModelBehavior for feed_tokens::ActiveModel {}
