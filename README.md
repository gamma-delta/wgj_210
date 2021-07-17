# Weekly Game Jam #210

The theme is: "Fragments"

---

## Devlog

This log has really big spoilers for the game in it!

### July 17th, Night 1 

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
