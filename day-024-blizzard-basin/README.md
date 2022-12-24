# Day 24: Blizzard Basin

With everything replanted for next year (and with elephants and monkeys to tend the grove), you and the Elves leave for the extraction point.

Partway up the mountain that shields the grove is a flat, open area that serves as the extraction point. It's a bit of a climb, but nothing the expedition can't handle.

At least, that would normally be true; now that the mountain is covered in snow, things have become more difficult than the Elves are used to.

As the expedition reaches a valley that must be traversed to reach the extraction site, you find that strong, turbulent winds are pushing small blizzards of snow and sharp ice around the valley. It's a good thing everyone packed warm clothes! To make it across safely, you'll need to find a way to avoid them.

Fortunately, it's easy to see all of this from the entrance to the valley, so you make a map of the valley and the blizzards (your puzzle input). For example:

```
#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#
```
The walls of the valley are drawn as #; everything else is ground. Clear ground - where there is currently no blizzard - is drawn as .. Otherwise, blizzards are drawn with an arrow indicating their direction of motion: up (^), down (v), left (<), or right (>).

The above map includes two blizzards, one moving right (>) and one moving down (v). In one minute, each blizzard moves one position in the direction it is pointing:

```
#.#####
#.....#
#.>...#
#.....#
#.....#
#...v.#
#####.#
```
Due to conservation of blizzard energy, as a blizzard reaches the wall of the valley, a new blizzard forms on the opposite side of the valley moving in the same direction. After another minute, the bottom downward-moving blizzard has been replaced with a new downward-moving blizzard at the top of the valley instead:

```
#.#####
#...v.#
#..>..#
#.....#
#.....#
#.....#
#####.#
```
Because blizzards are made of tiny snowflakes, they pass right through each other. After another minute, both blizzards temporarily occupy the same position, marked 2:

```
#.#####
#.....#
#...2.#
#.....#
#.....#
#.....#
#####.#
```
After another minute, the situation resolves itself, giving each blizzard back its personal space:

```
#.#####
#.....#
#....>#
#...v.#
#.....#
#.....#
#####.#
```
Finally, after yet another minute, the rightward-facing blizzard on the right is replaced with a new one on the left facing the same direction:

```
#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#
```
This process repeats at least as long as you are observing it, but probably forever.

Here is a more complex example:

```
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
```
Your expedition begins in the only non-wall position in the top row and needs to reach the only non-wall position in the bottom row. On each minute, you can move up, down, left, or right, or you can wait in place. You and the blizzards act simultaneously, and you cannot share a position with a blizzard.

In the above example, the fastest way to reach your goal requires 18 steps.

What is the fewest number of minutes required to avoid the blizzards and reach the goal?

Your puzzle answer was `343`.

--- Part Two ---
As the expedition reaches the far side of the valley, one of the Elves looks especially dismayed:

He forgot his snacks at the entrance to the valley!

Since you're so good at dodging blizzards, the Elves humbly request that you go back for his snacks. From the same initial conditions, how quickly can you make it from the start to the goal, then back to the start, then back to the goal?

In the above example, the first trip to the goal takes 18 minutes, the trip back to the start takes 23 minutes, and the trip back to the goal again takes 13 minutes, for a total time of 54 minutes.

What is the fewest number of minutes required to reach the goal, go back to the start, then reach the goal again?

Your puzzle answer was `960`.
