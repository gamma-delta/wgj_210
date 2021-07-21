# Weekly Game Jam #210

The theme is: "Fragments"

---

## Devlog

This log has really big spoilers for the game in it!

### July 16th, Night 1 

Guess I should write a devlog? I like reading those...

See [ideas.txt](./ideas.txt) for my thoughts as I came upwith this game.

Today I wrote the code to turn a 5x5 block of characters into a symbol, and identify
what kind of symbol it is. I also worked a little on the level/board structs.
And I wrote a test! It was very handy.

My current worry is that it's going to be way too hard to teach the player these systems.
I might have to put in some kind of heavy-handed tutorial ... but I'm hoping
I can do like The Witness and slowly introduce things to the player.
Also, I realize that the rules aren't actually that difficult once you know them,
but figuring them out probably will be. The Witness does a good job of making the puzzles
really satisfying after you know the rules. So, I'm hoping I can make the puzzles if not
the rules complicated and satisfying.

In that vein I'm not planning on letting the player twist the symbol nominoes around,
just translate them. (It's also easier to code that way...)

Speaking of easier to code, I require the big square at the beginning of every sentence
because I expect it will be easier to parse sentences that way.

Tomorrow, I hope to work on parsing whole sentences to check if they're valid.
Day after I think I'll finally get to work with the player interaction and moving
sentence fragments around the grid.

## July 17th, Night 2

I have written some untested code for parsing sentences. I've also written a ton of very very buggy code
for drawing the playfield.

I've written code to display infinite grids several times now and it always feels like I do it wrong
many times before I succeed. I am extremely tired right now so that's part of it.

Right now the position of the upper-left corner in world space is stored as my view position.
I'm having a lot of trouble with the conversions between world and pixel space...
I see why the `euclid` crate has its vectors require a marker type for the unit.
(I think it's called `euclid`.)

Writing the state machine for the grammar check was fun. I've written a lot of Racket lately (a Lisp
dialect) and I think the code-is-data mentality I had to practice made grammar checking a lot less painful.
I wouldn't want to use code-is-data for everything, I do think it's a little overhyped, but it
is certainly an effective tool.

I'm still on track with yesterday's schedule though! Tomorrow I hope to finish up the
horrid grid-display code, start displaying symbols, move them around, and check for grammar.
The day after I'll work on a level select screen ... I swear all of my games end up looking the same
at the end. The "textbox full of level names" approach is just so *effective* though.

Speaking of, now that I've refactored the textboxes to use `Span`s instead of absolutely
placed characters, it's going to be tricky to check which line I am clicking on.
I think I will add some kind of userdata `u128` to each `Span`, borrowing the idea from
the Rapier physics engine. That way I can store what line something is on, or use it as a
key into some hashmap if more associated information is needed.
(In another game I'm working on using Hecs ECS, each `Collider` and `RigidBody`'s userdata is
set to the entity handle serialized to bits.)

## July 18th, Night 3

I managed to write next to absolutely nothing today, I am very tired, and don't want to go to work
tomorrow.

Ugh.

## July 19th, Night 4

Forgot to write this last night so I'll do it now.

I decided to stop worrying about panning the display and made it a static grid.
I also have symbol drawing going alright.

I slow down a lot as I work on games. Too bad ... I need to get medicated for ADHD
or something I swear.

It's just not that fun to keep working on this game anymore. :(

## July 21st, Afternoon ... 6

Yes, I've given up working on this game. :(

Too bad... I feel like it could have been interesting. But I'm just too exhausted to work on it now.

If anyone wants to continue working on it for some reason, I'm pretty sure
parsing and all is implemented correctly. You just need a level-select screen and 
a bunch more levels. I can update the license to allow you use to use the few game-specific assets
(but please do not steal my opening screen, it should be easy enough to remove).
