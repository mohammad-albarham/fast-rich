# VHS tape for progress bar demo (full features)
Output docs/assets/progress.gif

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 900
Set Padding 20
Set FontSize 18
Set TypingSpeed 50ms

# Clean startup
Hide
Type "export PS1='$ '"
Enter
Type "clear"
Enter
Show

# Type the command to run the example
Type "cargo run --example progress_rich --release 2>/dev/null"
Sleep 500ms
Enter

# Wait for the example to finish (all demos)
Sleep 35s

# Pause at the end
Sleep 2s
