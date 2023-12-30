# Logger

this thing 

- tracks keystrokes on linux in a buffer, stores them as integer keycodes,
- every time buffer is filled converts raw data into ngram statistics and pushes to logfile
- data will be used for training hydraea model
- please use and send me data.flop
- afaik there shouldn't be any security degradations by using n grams up to quadrams to train a keyboard(i.e password showing up)

## Usage Instructions
- specify data log path in the constant variable 
- carbo build --release
- put file in /usr/bin
- run whenever feel like / startup script


THIS PROJECT IS NOT MALICIOUS I WANT TO MAKE COOL KEYBOARD LAYOUT 
