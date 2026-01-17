use tokio::sync::{MutexGuard, mpsc::Sender};

use crate::{App, AppAction, InputMode};

fn clamp_cursor(state: &mut MutexGuard<'_, App>) {
    let len = state.input.len();
    if state.cursor_position > len {
        state.cursor_position = len;
    }
}

fn move_cursor_word_forward(state: &MutexGuard<'_, App>) -> usize {
    let input = &state.input;
    let len = input.len();
    let mut pos = state.cursor_position;

    if pos >= len {
        return pos;
    }

    // If on a space, skip spaces
    while pos < len && input.chars().nth(pos).unwrap().is_whitespace() {
        pos += 1;
    }

    // Skip current word
    while pos < len && !input.chars().nth(pos).unwrap().is_whitespace() {
        pos += 1;
    }

    // Skip spaces to next word
    while pos < len && input.chars().nth(pos).unwrap().is_whitespace() {
        pos += 1;
    }

    pos.min(len)
}

fn move_cursor_word_backward(state: &MutexGuard<'_, App>) -> usize {
    let input = &state.input;
    let mut pos = state.cursor_position;

    if pos == 0 {
        return 0;
    }

    // Move back one char to start checking
    pos -= 1;

    // Skip spaces backwards
    while pos > 0 && input.chars().nth(pos).unwrap().is_whitespace() {
        pos -= 1;
    }

    // Skip word backwards
    while pos > 0 && !input.chars().nth(pos - 1).unwrap().is_whitespace() {
        pos -= 1;
    }

    pos
}

pub async fn handle_vim_keys(
    mut state: MutexGuard<'_, App>,
    c: char,
    tx_action: Sender<AppAction>,
) {
    match c {
        'i' => {
            state.mode = InputMode::Insert;
        }
        'I' => {
            state.cursor_position = 0;
            state.mode = InputMode::Insert;
        }
        'a' => {
            if state.cursor_position < state.input.len() {
                state.cursor_position += 1;
            }
            state.mode = InputMode::Insert;
        }
        'A' => {
            state.cursor_position = state.input.len();
            state.mode = InputMode::Insert;
        }
        'j' => {
            tx_action.send(AppAction::SelectNext).await.ok();
        }
        'k' => {
            tx_action.send(AppAction::SelectPrevious).await.ok();
        }
        'h' => {
            if state.cursor_position > 0 {
                state.cursor_position -= 1;
            }
        }
        'l' => {
            if state.cursor_position < state.input.len() {
                state.cursor_position += 1;
            }
        }
        'w' => {
            if let Some('d') = state.pending_command {
                let start = state.cursor_position;
                let end = move_cursor_word_forward(&state);
                if end > start {
                    state.input.drain(start..end);
                }
                state.pending_command = None;
            } else {
                state.cursor_position = move_cursor_word_forward(&state);
                clamp_cursor(&mut state);
            }
        }
        'b' => {
            if let Some('d') = state.pending_command {
                let end = state.cursor_position;
                let start = move_cursor_word_backward(&state);
                if end > start {
                    state.input.drain(start..end);
                    state.cursor_position = start;
                }
                state.pending_command = None;
            } else {
                state.cursor_position = move_cursor_word_backward(&state);
            }
        }
        'd' => {
            if let Some('d') = state.pending_command {
                state.input.clear();
                state.cursor_position = 0;
                state.pending_command = None;
            } else {
                state.pending_command = Some('d');
                state.last_command_time = std::time::Instant::now();
            }
        }
        ':' => {
            tx_action.send(AppAction::SelectEmoji).await.ok();
        }
        _ => {
            state.pending_command = None;
        }
    }
}
