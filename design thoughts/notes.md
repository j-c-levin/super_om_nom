om nom is the player

om nom has telkenesis abilities

when the player clicks on an object, om-nom attaches his powers to the object.  the object then follows the players mouse.

om nom is able to exert a certain amount of force on the object it's attached to.  

if the force is not enough to move the object, om nom is moved instead.  attaching to a 'static' object allows om-nom to launch himself around.

specifically:
	clicking an object attaches om-nom to it
		om nom is only attached to one object at a time
	the attached object has a force exerted on it in the direction of the mouse
	if the force is not enough to move the object, the force acts on om-nom instead in inverse
		if om-nom attempts to lift a static object, om-nom is lifted.
		if om-nom pulls a heavy object, om-nom is instead moved towards the object
		if om-nom pushes against the ground, om-nom is lifted up

it is as if a force is exterted out from om-nom and if the attached object cannot be moved it acts on om nom instead

but how can i do this without messing with the physics system?  ideally if om-nom attempts to pick up something very heavy but is weighed down, they are able to (slowly) lift the object itself.

if this wasn't telekenesis, what would it look like?  It would look like attaching a rope to an object.  you pull on the rope, and if the object is too heavy, you get pulled instead.  a pulley where if you can't lift the object, you lift yourself.

so, what physics joints do we have that can simulate a rope?

i don't think a joint does it.  we're not looking to lock forces.  we're looking to apply forces to everything.  perhaps that's it.  when you connect to something and apply a force, it applies a force to you too.  Maybe only the net of the force is applied.

yes, that's it.  let's say force is a linear factor of mass

		A 		B
mass	10		1		

A tries to lift B.  10-1 = 9 force factor on B

		A 		B
mass	10		11

A tries to lift B.  10-11=-1, -1 force factor on A.

-1 or 1?  For vertical movement it's just 1, but for x/y it's flipped.

Force on A:
	x: mass_a - mass_b
	y: abs(mass_a - mass_b)

Except this has to be physics driven - if A has less mass than B, but A is holding onto something static, A should move B instead.

So actually force is constant.  it is always applied to everyone.

Force on A:
	x: (mass_a - mass_b) * -1
	y: abs(mass_a - mass_b)

Force on B:
	x: mass_a - mass_b
	y: mass_a - mass_b

it's weird that A isn't pushed down.  not true action/reaction there.  how to reason this.

maybe thinking of it as a simple pulley is the right simulation actually.  Om nom is not pushing on the ground to lift, om nom is pulling down on a perfectly vertical pulley rope in order to apply a lifting force to another object.

Om nom is also pulling on a perfectly horizontal rope/pulley.  If om nom is pushing away or pulling into it experiences the opposite force.

but then what if om nom is holding onto a static point above it, it acts like a rope - om nom pulls down and goes up.

okay, test cases

om nom pulls a light object towards it
	om nom
		x: 10 - 1 * -1 = -9 (right)
		y: 0
	object
		x: 10-9 = 9 (left)
		y: 0

this seems odd, om nom shouldn't find itself flying towards a light object.

om nom pulls a heavy object towards it
	om nom
		x: 10 - 20 * -1 = 10 (left)
		y: 0
	heavy object
		x: 10 - 20 = 10 (right)
		y: 0

that don't check out. om nom should launch itself right.

what should it be?

om nom pulls a light object towards it
	om nom
		x: om nom experiences a very light force to the right
		y: 0
	object
		x: object experiences a strong force to the left
		y: 0

om nom pulls a heavy object towards it
	om nom
		x: om nom experiences a strong force to the right
		y: 0
	heavy object
		x: object experiences a strong force to the left
		y: 0

interesting, in both cases the object experiences the full force of om-nom, but in the second example it just won't be enough to really shift the object.  whereas for om nom it's different - when he pulls a light object om nom experiences a very light force.

it's just the object's mass.  god.  is it that simple?

when om-nom pulls a light object, om nom exerts its mass on the object and the object exerts its mass on om nom.

Force on A:
	x: mass_b in opposite direction
	y: mass_b in same direction

Force on B:
	x: mass_a in same direction
	y: mass_a in same direction

om nom (10) pulls a light object (1) towards it
	om nom
		x: 1 * -1 = -1 (right)
		y: 0
	light object
		x: 10 (left)
		y: 0

om nom pulls a heavy object towards it
	om nom
		x: 20 * -1 = 20 (right)
		y: 0
	heavy object
		x: 10 (left)
		y: 0

up and down?

om nom (10) lifts a light object (1)
	om nom
		x: 0
		y: 1 (up)
	light object
		x: 0
		y: 10(up)

om nom lifts a heavy object
	om nom
		x: 0
		y: 20 (up)
	heavy object
		x: 0
		y: 10 (up)

okay, so far much better.  

But what about blocking om-nom?  If it's closer in weight, what should happen?  Well, actually, the same.  Om-nom won't lift up because the resulting force is less.
	om nom
		x: 0
		y: 12
	object
		x: 0
		y: 10

om nom will receive net 2 force upwards, but the block on its head will prevent it from moving.  though it would be a fun mechanic to test out if putting objects on your head increases the body mass calculation and so you can lift more.

Okay, so objects put a force equal to their mass.

wait, what about om nom grabbing a heavy object and pushing down.  you would expect om nom to go up.  so it's not the same in the y direction, it's flipped. no, it's absoluted?

if om nom grabs a handle on the floor om nom needs to push it away to lift up.

if om nom grabs a handle on the door and lifts up, it lifts itself up.

so the y force is some component between absolute or flipped depending on the angle between om nom and itself.  Closer to 0 or 180 degrees results in inverted force.  90 and 270 degrees is absoluted.

that's a cosine wave.

0 degrees (directly above): pull down * inverted = upwards force
90 degrees (directly to right): pull up = upwards force
180 (directly below): pull down * inverted = upwards force
270 (directly left): pull up = upwards force

but that's the y component.  is the x the same?

0 degrees (directly above): pull towards = pull towards
90 degrees (directly to right): pull towards = pull towards
180 (directly below): pull away = pull towards
270 (directly left): pull towards = pull towards

so there's things going on here about what angle om nom is compared to the object and what happens?  just in the y component though.  x component is always in the opposite direction that the target object is being acted on.

okay, i think i have it.

wait maybe not cosine.  or a cosine that's locked between different values.  at 0 or 180 you want -1.  and 90 and 270 you want 1.

0: 	   -1
90: 	1
180:   -1
270: 	1
-cos(2x) (thank you ZacharyGPT)

Force on A:
x: mass_b in opposite direction
y: mass_b multiplied by -cos(2x) of angle between mass_a and mass_b (between 1 and -1)

Force on B:
x: mass_a towards target
y: mass_a towads target

is that right, should 45 degrees be 0?  so if om nom is attached to an object at 45 degrees to it and pulls up, it should push it where?  in theory it should push it down.  and if it's 135 (45 over horizon) it should push it up.

so it's actually nothing to do with angle, it's to do with y elevation.  if you are influencing an object on a different plane your y component is opposite to the one acting on the object.  on the other hand, if the object is on the same plane (real edge case?) then it's the same? hm.  might be getting into analysis paralysis with this and might just need to go for it and see.  maybe there's a 'band' around the player where things act in the same direction but otherwise it's the opposite.

yes!  that could be it.  same for x component - there is a band around the player where om nom experiences a force in the same direction as the object.  but outside of that band om nom experiences an opposite force

or...it turns out you can just always have the force acting in the opposite direction and it'll always just work and is easier to grasp.  maybe just need to play and see.