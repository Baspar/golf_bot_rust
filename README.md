# Golf Bot implementation in Rust

## Implementation and usage

`cargo run 1 2 3 4 / 8 1 50 `

The list of numbers are split in two parts:
- Before the `/` is the list of distances the bot can shot
- After the `/` is the list of distances to be checked

## Original subject

(Source: https://open.kattis.com/problems/golfbot)

Do you like golf? I hate it. I hate golf so much that I decided to build the ultimate golf robot, a robot that will never miss a shot. I simply place it over the ball, choose the right direction and distance and, flawlessly, it will strike the ball across the air and into the hole. Golf will never be played again.

Unfortunately, it doesn’t work as planned. So, here I am, standing in the green and preparing my first strike when I realize that the distance-selector knob built-in doesn’t have all the distance options! Not everything is lost, as I have 2 shots.

### Task

Given my current robot, how many holes will I be able to complete in 2 strokes or less?

### Input

The first line has one integer: `N`, the number of different distances the Golf Bot can shoot.
Each of the following `N` lines has one integer, `k_i`, the distance marked in position `i` of the knob.

Next line has one integer: `M`, the number of holes in this course.
Each of the following `M` lines has one integer, `d_j`, the distance from Golf Bot to hole `j`.

### Constraints

1≤`N`,`M`≤200000

1≤`k_i`,`d_j`≤200000

### Output

You should output a single integer, the number of holes Golf Bot will be able to complete. Golf Bot cannot shoot over a hole on purpose and then shoot backwards.

### Sample Output Explanation

Golf Bot can shoot 3 different distances (1, 3 and 5) and there are 6 holes in this course at distances 2, 4, 5, 7, 8 and 9. Golf Bot will be able to put the ball in 4 of these:
- The 1st hole, at distance 2, can be reached by striking two times a distance of 1.
- The 2nd hole, at distance 4, can be reached by striking with strength 3 and then strength 1 (or vice-versa).
- The 3rd hole can be reached with just one stroke of strength 5.
- The 5th hole can be reached with two strikes of strengths 3 and 5.

Holes 4 and 6 can never be reached.
