# VHS documentation
#
# Output:
#   Output <path>.gif               Create a GIF output at the given <path>
#   Output <path>.mp4               Create an MP4 output at the given <path>
#   Output <path>.webm              Create a WebM output at the given <path>
#
# Require:
#   Require <string>                Ensure a program is on the $PATH to proceed
#
# Settings:
#   Set FontSize <number>           Set the font size of the terminal
#   Set FontFamily <string>         Set the font family of the terminal
#   Set Height <number>             Set the height of the terminal
#   Set Width <number>              Set the width of the terminal
#   Set LetterSpacing <float>       Set the font letter spacing (tracking)
#   Set LineHeight <float>          Set the font line height
#   Set LoopOffset <float>%         Set the starting frame offset for the GIF loop
#   Set Theme <json|string>         Set the theme of the terminal
#   Set Padding <number>            Set the padding of the terminal
#   Set Framerate <number>          Set the framerate of the recording
#   Set PlaybackSpeed <float>       Set the playback speed of the recording
#   Set MarginFill <file|#000000>   Set the file or color the margin will be filled with.
#   Set Margin <number>             Set the size of the margin. Has no effect if MarginFill isn't set.
#   Set BorderRadius <number>       Set terminal border radius, in pixels.
#   Set WindowBar <string>          Set window bar type. (one of: Rings, RingsRight, Colorful, ColorfulRight)
#   Set WindowBarSize <number>      Set window bar size, in pixels. Default is 40.
#
# Sleep:
#   Sleep <time>                    Sleep for a set amount of <time> in seconds
#
# Type:
#   Type[@<time>] "<characters>"    Type <characters> into the terminal with a
#                                   <time> delay between each character
#
# Keys:
#   Backspace[@<time>] [number]     Press the Backspace key
#   Down[@<time>] [number]          Press the Down key
#   Enter[@<time>] [number]         Press the Enter key
#   Space[@<time>] [number]         Press the Space key
#   Tab[@<time>] [number]           Press the Tab key
#   Left[@<time>] [number]          Press the Left Arrow key
#   Right[@<time>] [number]         Press the Right Arrow key
#   Up[@<time>] [number]            Press the Up Arrow key
#   Down[@<time>] [number]          Press the Down Arrow key
#   PageUp[@<time>] [number]        Press the Page Up key
#   PageDown[@<time>] [number]      Press the Page Down key
#   Ctrl+<key>                      Press the Control key + <key> (e.g. Ctrl+C)
#
# Display:
#   Hide                            Hide the subsequent commands from the output
#   Show                            Show the subsequent commands in the output

Output base.gif

Require tisq

Set Shell "bash"
Set TypingSpeed 0.03
Set FontSize 28
Set Width 2400
Set Height 1400

Sleep 500ms
Type "tisq" Sleep 500ms Enter

Sleep 1s
Type "a"

Type "my-postgres"
Down
Type "postgres://postgres:kkiihhyywwfe1223@postgres:5432/test"

Sleep 1s
Enter

Sleep 1s

Right
Sleep 1s
Down
Sleep 1s
Right
Sleep 1s

Down
Sleep 0.3

Down
Sleep 1s

Type "q"

Sleep 1

Type "select * from test"

Sleep 1

Ctrl+e

Sleep 4

Ctrl+n
Sleep 1

Down
Sleep 0.5

Down
Sleep 0.5

Down
Sleep 0.5

PageDown
Sleep 3

Ctrl+n
Sleep 1

Up
Sleep 1

Type "q"
Sleep 1

Type "select datname, oid, encoding, datallowconn from pg_catalog.pg_database;"
Sleep 2

Ctrl+e
Sleep 3

Ctrl+l
Sleep 1

Enter
Type "ds"
Sleep 2

Alt+k
Sleep 2

Ctrl+e
Sleep 7


Ctrl+n
Sleep 1

Ctrl+n
Sleep 1

Down
Sleep 1

Right
Sleep 1

Down
Sleep 0.5

Down
Sleep 0.5

Down
Sleep 0.5

Right
Sleep 10


