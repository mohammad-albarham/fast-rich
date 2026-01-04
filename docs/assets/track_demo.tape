# VHS tape for track() iterator demo
Output docs/assets/track_demo.gif

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 300
Set Padding 20
Set FontSize 20
Set TypingSpeed 50ms

# Clean startup
Hide
Type "export PS1='$ '"
Enter
Type "clear"
Enter
Show

# Type the command to run the example
Type "cargo run --example docs_track --release 2>/dev/null"
Sleep 500ms
Enter

# Wait for the example to finish
Sleep 4s

# Pause at the end
Sleep 1s
