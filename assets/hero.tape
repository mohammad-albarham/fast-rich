# VHS tape for hero demo
Output docs/assets/hero.gif

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 600
Set Padding 20
Set FontSize 28
Set TypingSpeed 100ms

# Clean startup
Hide
Type "export PS1='$ '"
Enter
Type "clear"
Enter
Show

# Type the command to run the hero example
Type "cargo run --example hero"
Sleep 500ms
Enter

# Wait for the output
Sleep 4s

# Pause at the end
Sleep 3s
