# VHS tape for console basics demo
Output docs/assets/console_basics.gif

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 500
Set Padding 20
Set FontSize 24
Set TypingSpeed 60ms
Set LoopOffset 100%

# Clean startup
Hide
Type "export PS1='$ '"
Enter
Type "clear"
Enter
Show

# Type the command to run the console example
Type "cargo run --example console_print"
Sleep 500ms
Enter

# Wait for the output
Sleep 5s

# Pause at the end before loop
Sleep 2s
