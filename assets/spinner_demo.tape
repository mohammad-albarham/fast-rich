# Spinner Styles Demo
# Creates a GIF showing different spinner styles
Output docs/assets/spinner_demo.gif

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 800
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
Type "cargo run --example spinner_column_demo"
Sleep 500ms
Enter

# Wait for the demo to complete
Sleep 25s

# Pause at the end
Sleep 2s
