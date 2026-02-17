# Print Macros Demo
# Creates a GIF showing the drop-in print macro feature
Output docs/assets/print_macros.gif

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 600
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
Type "cargo run --example print_shadowing"
Sleep 500ms
Enter

# Wait for the output
Sleep 6s

# Pause at the end
Sleep 2s
