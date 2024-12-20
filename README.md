# Evolution Simulation

This project is mostly just a fun Rust-learning exercise, but the idea is to simulate evolution of little
pixel creatures over time as a kind of art piece. It's not really a fully formed idea yet, so many details tbd
as I go along :)

Basic idea is that each little pixel creature has a small neural network
"brain" that allows it to take in input and perform an action in response at every step of the sim.
If a creature lives long enough, it'll reproduce spreading it's brain structure to the next generation (with
optional random mutations). The end result should allow the user to play with a whole bunch of parameters to 
see the effect on the population.

The visualization uses the [macroquad](https://macroquad.rs/) library, and yeah I realize the UI is horrible lol.
I'm just going for basic functionality at this point. In the below screenshot, the creatures are blue, the food pieces
are green, and the black spaces are "walls".

![example screenshot](images/example_screenshot1.jpg)


