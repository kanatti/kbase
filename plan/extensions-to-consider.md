# Pi Extensions to Consider

Extensions from `~/Code/pi-mono/packages/coding-agent/examples/extensions/` that might be useful.

## High Priority

### Safety & Permission
- **`confirm-destructive.ts`** - Confirms before destructive session actions (clear, switch, fork)
- **`protected-paths.ts`** - Blocks writes to protected paths (.env, .git/, node_modules/)
- **`dirty-repo-guard.ts`** - Prevents session changes with uncommitted git changes

### Git Integration
- **`git-checkpoint.ts`** - Creates git stash checkpoints at each turn (for code restoration on fork)
- **`auto-commit-on-exit.ts`** - Auto-commits on exit using last assistant message

### Commands & Workflow
- ✅ **`handoff.ts`** - Transfer context to a new focused session via `/handoff <goal>` (added to pipi)
- **`preset.ts`** - Named presets for model/thinking/tools via `--preset` and `/preset`
- **`tools.ts`** - Interactive `/tools` command to enable/disable tools with session persistence
- **`bookmark.ts`** - Bookmark entries with labels for `/tree` navigation

### Custom Tools
- **`truncated-tool.ts`** - Wraps ripgrep with proper output truncation (50KB/2000 lines)
- **`tool-override.ts`** - Override built-in tools (e.g., add logging/access control to `read`)
- **`subagent/`** - Delegate tasks to specialized subagents with isolated context windows

## Medium Priority

### System Prompt & Compaction
- **`claude-rules.ts`** - Scans `.claude/rules/` folder and lists rules in system prompt
- **`trigger-compact.ts`** - Triggers compaction when context usage exceeds 100k tokens

### UI Enhancement
- **`status-line.ts`** - Shows turn progress in footer
- **`custom-footer.ts`** - Custom footer with git branch and token stats
- **`notify.ts`** - Desktop notifications when agent finishes (Ghostty, iTerm2, WezTerm)

### Development Tools
- **`summarize.ts`** - Summarize conversation with different model
- **`question.ts`** - Demonstrates `ctx.ui.select()` for asking the user questions
- **`questionnaire.ts`** - Multi-question input with tab bar navigation

## Low Priority (Interesting but Maybe Not Needed)

### Advanced Workflow
- **`plan-mode/`** - Claude Code-style plan mode for read-only exploration
- **`sandbox/`** - OS-level sandboxing using `@anthropic-ai/sandbox-runtime`
- **`ssh.ts`** - Delegate all tools to remote machine via SSH
- **`interactive-shell.ts`** - Run interactive commands (vim, htop) with full terminal
- **`inline-bash.ts`** - Expands `!{command}` patterns in prompts
- **`input-transform.ts`** - Transform user input before it's sent to LLM

### System Prompt Customization
- **`pirate.ts`** - Demonstrates `systemPromptAppend` to dynamically modify system prompt
- **`system-prompt-header.ts`** - Add custom header to system prompt
- **`custom-compaction.ts`** - Custom compaction that summarizes entire conversation

### UI & Display
- **`custom-header.ts`** - Custom header via `ctx.ui.setHeader()`
- **`widget-placement.ts`** - Shows widgets above and below the editor
- **`modal-editor.ts`** - Custom vim-like modal editor via `ctx.ui.setEditorComponent()`
- **`rainbow-editor.ts`** - Animated rainbow text effect via custom editor
- **`titlebar-spinner.ts`** - Braille spinner animation in terminal title while agent is working
- **`message-renderer.ts`** - Custom message rendering with colors and expandable details
- **`overlay-test.ts`** - Test overlay compositing with inline text inputs
- **`overlay-qa-tests.ts`** - Comprehensive overlay QA tests

### Session Management
- **`session-name.ts`** - Name sessions for the session selector via `setSessionName`

### Inter-Extension Communication
- **`event-bus.ts`** - Inter-extension communication via `pi.events`

### Advanced Tools
- **`antigravity-image-gen.ts`** - Generate images via Google Antigravity with save-to-disk modes
- **`todo.ts`** - Todo list tool + `/todos` command with custom rendering and state persistence
- **`qna.ts`** - Extracts questions from last response into editor via `ctx.ui.setEditorText()`
- **`send-user-message.ts`** - Demonstrates `pi.sendUserMessage()` for sending user messages from extensions
- **`timed-confirm.ts`** - Demonstrates AbortSignal for auto-dismissing dialogs

### Command Examples
- **`commands.ts`** - Simple command registration examples
- **`shutdown-command.ts`** - Adds `/quit` command demonstrating `ctx.shutdown()`

### Monitoring & Hooks
- **`bash-spawn-hook.ts`** - Hook into bash command execution
- **`file-trigger.ts`** - Watches a trigger file and injects contents into conversation

### Custom Providers
- **`custom-provider-anthropic/`** - Custom Anthropic provider with OAuth
- **`custom-provider-gitlab-duo/`** - GitLab Duo provider
- **`custom-provider-qwen-cli/`** - Qwen CLI provider with OAuth device flow

### Dynamic Resources
- **`dynamic-resources/`** - Loads skills, prompts, and themes using `resources_discover`

### Platform Integration
- **`mac-system-theme.ts`** - Syncs pi theme with macOS dark/light mode

### Demos & Examples
- **`hello.ts`** - Minimal custom tool example
- **`rpc-demo.ts`** - Exercises all RPC-supported extension UI methods
- **`with-deps/`** - Extension with its own package.json and dependencies

## Games (Just for Fun)
- **`doom-overlay/`** - DOOM game running as an overlay at 35 FPS
- **`snake.ts`** - Snake game with custom UI
- **`space-invaders.ts`** - Space Invaders

## Already Implemented
- ✅ **`permission-gate.ts`** - Already in pipi
- ✅ **`session-context/`** - Similar to our session-context extension (save/continue)

## Notes

**Key Patterns to Remember:**
- Use `StringEnum` for string parameters (required for Google API compatibility)
- Store state in tool result `details` for proper forking support
- Use `session_start` event to reconstruct state from session history

**Location:** All examples in `~/Code/pi-mono/packages/coding-agent/examples/extensions/`
