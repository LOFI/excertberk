# Systems

Jotting down some general observations from watching 
https://www.youtube.com/watch?v=fRgMCtaWoSU
in the hopes of describing some of the systems at work in the game.


## Movement/Control

http://www.thegameisafootarcade.com/wp-content/uploads/2017/02/Excitebike-Game-Manual.pdf

* Accelerate/Break buttons change speed, but player appears in fixed horizontal
  screen location (with exceptions for start/end of race).
* Lane switching up/down.
* Tiles affect movement speed (dirt, grass, humps, ramps, boosts).
* Wheelies!

## Tumbles

* Bikes tumble, causing the racer to have to recover from the spill before they
  can continue in the race.
* Tumbles can be caused by:
  * the front end of a bike coming into contact with the rear of another.
  * a wheelie gone too far.
  * catching some air and landing on the tip of a ramp between the wheels.
  * landing at a bad angle after a jump (what are the ranges?)


## Camera

As mentioned in the movement section, the camera seems locked to the player
X position except for at the start/end of the race where the X is clamped to 
some min/max.

## Tile types

* Grass
* Track
* Gravel/Dirt
* Hump
* Ramp (various angles/sizes)
* Shelf/Plateau (raised, but level)
