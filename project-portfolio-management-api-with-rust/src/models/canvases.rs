//! Canvas (LT-1) and comment/reaction (LT-4) model helpers: the `ActiveModelBehavior` impls.

use loco_rs::prelude::*;

use super::_entities::{
    canvas_items, canvases, comment_edits, comments, notification_prefs, plan_modules, reactions,
};

impl ActiveModelBehavior for canvases::ActiveModel {}
impl ActiveModelBehavior for canvas_items::ActiveModel {}
impl ActiveModelBehavior for comments::ActiveModel {}
impl ActiveModelBehavior for comment_edits::ActiveModel {}
impl ActiveModelBehavior for reactions::ActiveModel {}
impl ActiveModelBehavior for notification_prefs::ActiveModel {}
impl ActiveModelBehavior for plan_modules::ActiveModel {}
