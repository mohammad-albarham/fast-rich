# VHS tape for running all examples
Output docs/assets/run_all.gif

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 900
Set Padding 20
Set FontSize 22
Set TypingSpeed 100ms
Set PlaybackSpeed 0.5

# Clean startup
Hide
Type "export PS1='$ '"
Enter
Type "clear"
Enter
Show

# Type the command to run all examples
Type "./scripts/run_all_examples.sh"
Sleep 500ms
Enter

# This script runs 13 examples. We need to wait for it to finish.
# We increase the sleep time to allow the user to see the results of each example as they scroll by.
# We use PlaybackSpeed 0.5 to make it 2x slower total.
Sleep 55s

# Pause at the end to show the summary table clearly
Sleep 15s
