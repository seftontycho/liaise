# Mechanics

Liaise is a turn based game. Each turn of the game consists of a number of phases, each of which is executed in sequence. The phases are as follows:

1. **Sense** - Each drone senses its environment.
2. **Send** - Each drone sends messages to all of its neighbors.
3. **Recieve** - Each drone recieves messages from its neighbors.
4. **Act** - Each drone perfroms an action based on the previous phases.

The player must write code that will control the drones in each of these phases. The code that the player writes will be executed by each drone in turn, and the drones will execute this code autonomously.

## Sense

Each drone has a field-of-view, an area around them, that they can sense.
For the intial release, the information that a drone can sense will be perfect, IE a drone can rely that the information it recieves is always accurate.

The information that a drone can sense will be as follows:

- The position (relative to itself) and state of all other entities in its field-of-view.

## Send

Each drone can send messages to all of its neighbors.
Messages cannot be targeted at a specific drone, they are broadcast to all neighbors.

We will allow the user to construct their own message type. The current proposal is that a message will be a type that implements Serde's Serialize and Deserialize traits.

## Recieve

Each drone recieves messages from all of its neighbors.

## Act

Each drone can perform one action based on the information that it has sensed and the messages that it has recieved.

The actions that a drone can perform will be as follows:

- Do nothing.
- Move to a new position.
- Change the state of certain entities in its field-of-view.
