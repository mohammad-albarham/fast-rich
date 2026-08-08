# VHS tape for markup syntax demo - minimal version
Output docs/assets/markup_demo.gif

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set Width 1000
Set Height 600
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

# Type the command 
Type "cargo run --example docs_markup"
Sleep 500ms
Enter

# Wait for output to display
Sleep 5s

# Pause at the end
Sleep 2s
