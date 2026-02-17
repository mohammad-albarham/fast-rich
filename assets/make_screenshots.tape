# VHS tape for make screenshots automation demo
Output docs/assets/make_screenshots.gif

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 800
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

# Type the command to run the automation
Type "make screenshots"
Sleep 500ms
Enter

# Wait for the process to complete (it takes a while to generate 10+ images)
Sleep 15s

# Pause at the end to show it finished
Sleep 5s
