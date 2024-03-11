it's kind of working, but there's also some funny edge cases.  do i want to fix them or just move on?

i feel like just move on - next ones will probably involve clamping distance etc.

do i want it to have distance clamping?  i don't think distance clamping, but the player shouldn't whip off the screen.  but is that a new example?  sure, why not, feels right.

so the next example should be about refining player controls and setting up a new playground space

1. create a new playground with monkey bars, platforms, and objects of different sizes - s, m, l, xl, static (try and define these as constants)
2. players don't whip off the screen and continue accelerating if they're holding something
3. you should be able to have fun bouncing yourself around and putting the 'mess' into buckets.

--

* work out the locations for the walls and get them spawning
* add in monkey bars and baskets

--

* playground controls work with click and drag/touch and drag ✅
* wasm build
* cap forces on om nom ✅
* forces on om nom should be multiplied by the object's density (see ideas) ✅