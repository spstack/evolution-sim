# Evolution Simulation

This project is mostly just a fun Rust-learning exercise, but the idea is to simulate evolution of little
pixel creatures over time as a kind of art piece. It's not really a fully formed idea yet, so many details tbd
as I go along :)

Basic idea is that each little pixel creature has a small neural network
"brain" that allows it to take in input and perform an action in response at every step of the sim.
If a creature lives long enough, it'll reproduce spreading it's brain structure to the next generation (with
optional random mutations). The end result should allow the user to play with a whole bunch of parameters to 
see the effect on the population.

The visualization uses the [macroquad](https://macroquad.rs/) library, and yeah I realize the UI is kind of horrible.
I'm just going for basic functionality at this point.

![example screenshot](images/evolution_sim.gif)

The basic rules are that each creature has an energy level, and when that energy reaches zero or the creature reaches a maximum age,
the creature dies. A creature performs one action each step, and different actions require differing amounts of energy.
 When a creature dies, it leaves behind a food piece. If the creature consumes a food piece, they get 
a user-defined amount of energy back. If the creature reaches a certain level of energy, they will automatically reproduce as the next action. A reproduction event causes a copy of the creature to be made,
but each weight/bias parameter in the creature's neural network brain has a certain probability of being randomly mutated.
This allows creatures with effective brains to survive and keep spreading their "DNA".


Each creature's "brain" has input neurons that can sense:
* Current energy level
* Current age
* Last action
* Color and distance of object in front of it
* Current orientation

Then the neural net is evaluated, and the creature chooses one output "Action" to perform at each step. The available actions are:
* Stay
* Move Forwards/Backwards/Left/Right
* Rotate Clockwise/Counter-Clockwise 
* Reproduce
* Kill

If a creature chooses the "kill" action, it will only work if there is another creature directly in front of it. That target creature is automatically consumed for food and the hunting creature is turned a slightly more red color. This color is reflected in the vision of other creatures, so that others can tell the difference between a violent and non-violent creature. This allows for various survival strategies to emerge. The kill action still costs energy regardless of whether it is successful.




