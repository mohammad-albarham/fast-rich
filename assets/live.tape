# VHS tape for live display demo
Output docs/assets/live.gif

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 600
Set Padding 20
Set FontSize 24
Set TypingSpeed 100ms

# Clean startup
Hide
Type "export PS1='$ '"
Enter
Type "clear"
Enter
Show

# Type the command to run the example
Type "cargo run --example live_clock"
Sleep 500ms
Enter

# Wait for some updates to show
Sleep 8s

# Stop the example (it likely has a 'q' to quit or just wait)
Type "q"
Sleep 1s

# Pause at the end
Sleep 2s
