# tmux-botdomo

Majordomo of AI assistants living in [tmux](https://github.com/tmux/tmux) sessions.

Automatically detects Claude Code, Gemini, and Codex running in tmux panes and allows you to send context directly from vim to the right AI assistant.

## Installation

```bash
cargo install tmux-botdomo
```

## Setup

### 1. Start the daemon

Add to your `.tmux.conf`:

```bash
set-hook -g server-start "run-shell 'tbdmd start'"
```

Note that `session-closed` hook is currently broken in tmux 3.5a and doesn't get triggered as expected. Therefore, `tbdmd` automatically shuts down when it detects the tmux session no longer exists.

### 2. Configure Neovim

```lua
vim.keymap.set('v', '<leader>tb', function()
  vim.cmd('normal! "zy')
  local context = vim.fn.getreg('z')
  vim.fn.system('tbdm send ' .. vim.fn.shellescape(context))
end)
```

This essentially mimics yanking.

## Usage

1. Select code in vim/Neovim.
2. Press your configured keybinding.
3. Selected conetext is sent to the detected AI assistant in your tmux session, **based on the working directories**.

## Commands

```bash
tbdm status     # Check daemon status
tbdm send        # Send context to AI assistant (typically called from vim)
```
