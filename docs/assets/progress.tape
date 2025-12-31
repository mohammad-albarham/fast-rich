# VHS tape for progress bar demo
Output docs/assets/progress.gif

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
Type "cargo run --example progress_bar"
Sleep 500ms
Enter

# Wait for the example to finish and show the output
Sleep 5s

# Pause at the end
Sleep 2s
