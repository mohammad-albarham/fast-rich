# VHS tape: demonstration of the fixes from the 2026-09-27 review.
# Produces a GIF and an MP4 of the affected functionalities running in a
# real terminal. Requires the examples to be prebuilt:
#   cargo build --all-features --examples
Output docs/assets/fixes_showcase.gif
Output docs/assets/fixes_showcase.mp4

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set Width 1280
Set Height 900
Set Padding 20
Set FontSize 19
Set TypingSpeed 35ms
Set LoopOffset 0%

Hide
Type "export PS1='$ '"
Enter
Type "clear"
Enter
Type "export FAST_RICH_DEMO_SLOW=1"
Enter
Show

# ── Part 1: the before/after showcase ────────────────────────────────
Type "cargo run -q --all-features --example fixes_showcase"
Enter
Sleep 26s

# ── Part 2: prompt EOF safety (old code hung forever here) ──────────
Type "printf '' | cargo run -q --all-features --example prompt_eof"
Enter
Sleep 5s

# ── Part 3: real auto-refresh thread animating live bars ────────────
# (no manual refresh() calls — the fixed background thread does all redraws)
Type "clear"
Enter
Type "cargo run -q --all-features --example auto_refresh_demo"
Enter
Sleep 8s
Type "clear"
Enter
Sleep 300ms
