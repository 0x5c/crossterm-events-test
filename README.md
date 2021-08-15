# a gizmo to test polling times in crossterm events

yep, that's pretty much the it

it waits, it polls, says whether or not an event was got, rinse, repeat

`-w` for the waiting time, `-p` for the polling time, both in microseconds  
defaults are 2s and 1s, respectively

ctrl+c makes it exit


## running it

clone repo, build with cargo, serve with keypresses
