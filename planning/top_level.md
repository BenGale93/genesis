# Genesis Plan

This project aims to be a performant life simulator written in Rust.

## Brain

The organisms "Brain" will be modelled using a Neuroevolution of augmenting
topologies (NEAT) methodology.

It will take a set of inputs and fire a set of outputs. The inputs will be
stimulations from the environment and outputs will be actions the organism can
take.

The brain will evolve and that will involve the following:

* Add new connections.
* Add new nodes with given functions.
* Change the weights between nodes.
* Remove connections and any floating nodes.

Each new change to the structure will be recorded and given an innovation
number.

Sexual reproduction will involve a more complex crossover process.

### Inputs

* Constant.
* Energy levels.
* Maturity.
* Healthiness.
* Speed.
* Heartbeat.
* Time alive.
* Internal timer. Used to space out reproduction for example.

The hardest input to model is sight.

* Number of visible food sources.
* Angle to closest food source.
* Distance to closest food source.
* Number of visible organisms.
* Angle to closest organism.
* Distance to closest organism.

### Outputs

* Accelerate.
* Rotate.
* Reproduce.
* Eat.
* Reset internal timer.

## Body

The body will be modelled using a genome. Some characteristics will be a function
of a single gene whereas others will be a more complex combination. For example,
size, speed etc. should be linked to energy needs.

The body will expend energy on:

* Growing.
* Producing offspring.
* Moving.
* Thinking. More complex brains should cost more.
* Stay alive.

### Characteristic list

* Size.
* Speed.
* Strength
* Mutation size.
* Mutation chance.
* Sight range
* Sight angle.
* Metabolism cost.
* Energy passed to offspring.
* Initial size of offspring.

## Environment

The environment will be initially modelled as a system with a fixed energy
budget. This will reduce the scope for runaway population growth. In other
words, the total energy contained in all the organisms and the food will remain
fixed. As organisms die, the environment will be free to generate new food.

The implication of this is that each organism will have a base energy store
that it will release on death. It will be contingent on size and will also
imply that a proportion of energy must be reserved for growing and producing
offspring

The environment will be responsible for: generating food, spawning new random
organisms if numbers get low and recycling dead organisms.
