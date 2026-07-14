use leptos::prelude::*;
use ykcode_core::Document;

use crate::EditorCtx;

pub const MAX_HISTORY: usize = 50;

/// Capture a snapshot of the document BEFORE a mutation.
/// Call this before every ctx.document.update(...) that represents a user action.
pub fn push_history(ctx: EditorCtx) {
    if ctx.history_paused.get_untracked() {
        return;
    }
    let snapshot = ctx.document.get_untracked();
    ctx.undo_stack.update(|stack| {
        stack.push(snapshot);
        if stack.len() > MAX_HISTORY {
            stack.remove(0);
        }
    });
    ctx.redo_stack.set(Vec::new());
}

/// Mutation wrapper — captures history then applies the mutation.
/// Use this instead of bare ctx.document.update() for user actions.
pub fn with_history(ctx: EditorCtx, f: impl FnOnce(&mut Document)) {
    push_history(ctx);
    ctx.document.update(f);
}

pub fn undo(ctx: EditorCtx) {
    let snapshot = ctx.undo_stack.write().pop();
    let Some(prev) = snapshot else { return };
    let current = ctx.document.get_untracked();
    ctx.history_paused.set(true);
    ctx.redo_stack.update(|stack| stack.push(current));
    ctx.document.set(prev);
    ctx.history_paused.set(false);
}

pub fn redo(ctx: EditorCtx) {
    let snapshot = ctx.redo_stack.write().pop();
    let Some(next) = snapshot else { return };
    let current = ctx.document.get_untracked();
    ctx.history_paused.set(true);
    ctx.undo_stack.update(|stack| stack.push(current));
    ctx.document.set(next);
    ctx.history_paused.set(false);
}

pub fn can_undo(ctx: EditorCtx) -> impl Fn() -> bool {
    move || !ctx.undo_stack.with(|s| s.is_empty())
}

pub fn can_redo(ctx: EditorCtx) -> impl Fn() -> bool {
    move || !ctx.redo_stack.with(|s| s.is_empty())
}
