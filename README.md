# Ai Wargame

You can assume the following initial configuration on a 5x5 board:
```
      0   1   2   3   4
 A:  dA9 dT9 dF9  .   .
 B:  dT9 dP9  .   .   .
 C:  dF9  .   .   .  aP9
 D:   .   .   .  aF9 aV9
 E:   .   .  aP9 aV9 aA9
```
Attacker has 1 AI, 2 Viruses, 2 Programs, 1 Firewall

Defender has 1 AI, 2 Techs, 2 Firewalls, 1 Program

Attacker starts.

## Rules:

Movement and other actions cannot be diagonal (only left, top, right, bottom). Only the effect of self-destruction affects diagonals.

Attacker always moves up or left.
Defender always moves down or right.
Exception: Tech and Virus can always move in any direction.
Actual movement is possible only if destination is empty.

Instead of moving, a unit can
- attack an adjacent opposing unit (no diagonals)
- repair an adjacent friendly unit (no diagonals)
- self-destruct and damage surrounding units (diagonals included)

Self-destruction removes the unit and inflicts 2 points of damage to all 8 surrounding units (if present). This includes diagonals and friendly units.

For damage and repair actions, refer to the appropriate tables for the results (it varies according to the units involved).

In order for an attack or repair action to be valid, it must result in a change of health on either side. This means a player cannot pass their turn by performing a zero-point repair operation.

All units start with 9 points of health and are removed after a combat exchange if their health is 0.
Health cannot go above 9 or below 0.
When combat occurs between opposing units (X attacks Y), X damages Y but Y also damages X according to the same damage tables (mutual combat).
When repair happens between friendly units, the initiator repairs the target (not bi-directional).
You cannot repair an opposing unit or attack a friendly unit.

Units are said to be engaged in combat if an opposing unit is adjacent (in any of the 4 directions).
A unit cannot move if engaged in combat, except the Virus and the Tech.

The game ends when any of these conditions is true:

- a player loses their AI
- a player cannot perform any action
- a pre-determined number of moves has been reached (default is 100)

To determine the winner at the end of the game, these rules are applied:

- a player wins if their AI is alive when the other AI is destroyed
- a player loses if no action can be performed
- the defender wins

## Tables

### Damage tables for attacker to defender
```
Damage table:
 from / to        AI     Virus      Tech  Firewall   Program
        AI         3         3         3         1         3
     Virus         9         1         6         1         6
      Tech         1         6         1         1         1
  Firewall         1         1         1         1         1
   Program         3         3         3         1         3
```
A few important details:

- the Virus can destroy the AI in 1 attack (9 points)
- the Firewall is good at absorbing attacks but bad at damaging other units
- Tech and Virus are equal in combat against each other
- Virus is very offensive
- Tech is very defensive

### Repair tables for friendly units
```
Repair table:
 from / to        AI     Virus      Tech  Firewall   Program
        AI         0         1         1         0         0
      Tech         3         0         0         3         3
```
As you can see, the Tech can repair AI, Firewall and Program by 3 points.
The AI can repair the Virus and Tech by 1 point.
These are the only allowed repairs.

### Common strategies

- A Tech unit behind AI, Firewall, Program can make them much stronger.
- A Virus can move back to it's AI to get slowly repaired.
- A Tech can move around in any direction to help units in need of repair.
- The AI cannot move back so it can be blocked by some units and then destroyed by a Virus.
- Self-destruction is a valid tactic for a unit with low health that is surrounded by opposing units.

### Heuristics

- The 2 players have different goals and units so using specialized heuristics for each one will help.

### How to move

Moves are expressed as a pair of coordinates (from, to). Depending on the contents of the cells at those coodinates, they will trigger different actions (movement, attack, repair). When both corrdinates are the same, it means self-destruction.

With a console-based text interface, coordinates are expressed as a letter and a number for row and column. Rows start at A and columns at 0. Example: B3 C3 means unit at (1,3) acts on (2,3).

In a simple mouse-driven interface, you click on the starting cell and then the destination cell (or drag and drop if that is implemented).

### Example output (AI vs AI with full debug traces, manually shortened)

```

0/100 moves played
Max search depth: 6
Max search time: 5.0 sec
Total evals at each depth: {}
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9 dP9  .   .   . 
 C:  dF9  .   .   .  aP9
 D:   .   .   .  aF9 aV9
 E:   .   .  aP9 aV9 aA9

Attacker: move from E2 to D2
Compute time: 0.3 sec
Average depth: 5.1
Heuristic score: -881

1/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 33, 6: 45338, 2: 4, 1: 1, 5: 817, 4: 146}
Average eval depth: 6.0
Average eval time: 0.3
Average branching factor: 4.8
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9 dP9  .   .   . 
 C:  dF9  .   .   .  aP9
 D:   .   .  aP9 aF9 aV9
 E:   .   .   .  aV9 aA9

Defender: move from C0 to C1
Compute time: 0.1 sec
Average depth: 4.4
Heuristic score: -497

2/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 67, 6: 45338, 2: 10, 1: 2, 5: 11460, 4: 319}
Average eval depth: 5.8
Average eval time: 0.2
Average branching factor: 4.9
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9 dP9  .   .   . 
 C:   .  dF9  .   .  aP9
 D:   .   .  aP9 aF9 aV9
 E:   .   .   .  aV9 aA9

Attacker: move from E3 to E2
Compute time: 0.0 sec
Average depth: 3.6
Heuristic score: -881

3/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 103, 6: 45338, 2: 15, 1: 3, 5: 11460, 4: 2256}
Average eval depth: 5.7
Average eval time: 0.1
Average branching factor: 4.9
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9 dP9  .   .   . 
 C:   .  dF9  .   .  aP9
 D:   .   .  aP9 aF9 aV9
 E:   .   .  aV9  .  aA9

Defender: move from C1 to D1
Compute time: 0.1 sec
Average depth: 4.3
Heuristic score: -700

4/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 143, 6: 45338, 2: 23, 1: 4, 5: 27698, 4: 2482}
Average eval depth: 5.6
Average eval time: 0.1
Average branching factor: 5.0
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9 dP9  .   .   . 
 C:   .   .   .   .  aP9
 D:   .  dF9 aP9 aF9 aV9
 E:   .   .  aV9  .  aA9

Attacker: move from C4 to B4
Compute time: 0.0 sec
Average depth: 3.6
Heuristic score: -1066

5/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 181, 6: 45338, 2: 28, 1: 5, 5: 27698, 4: 4588}
Average eval depth: 5.5
Average eval time: 0.1
Average branching factor: 5.0
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9 dP9  .   .  aP9
 C:   .   .   .   .   . 
 D:   .  dF9 aP9 aF9 aV9
 E:   .   .  aV9  .  aA9

Defender: move from B1 to C1
Compute time: 0.1 sec
Average depth: 4.4
Heuristic score: -901

6/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 215, 6: 45338, 2: 31, 1: 6, 5: 40289, 4: 4759}
Average eval depth: 5.4
Average eval time: 0.1
Average branching factor: 5.1
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9  .   .   .  aP9
 C:   .  dP9  .   .   . 
 D:   .  dF9 aP9 aF9 aV9
 E:   .   .  aV9  .  aA9

Attacker: move from B4 to A4
Compute time: 0.0 sec
Average depth: 3.6
Heuristic score: -1261

7/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 252, 6: 45338, 2: 36, 1: 7, 5: 40289, 4: 6728}
Average eval depth: 5.4
Average eval time: 0.1
Average branching factor: 5.1
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .  aP9
 B:  dT9  .   .   .   . 
 C:   .  dP9  .   .   . 
 D:   .  dF9 aP9 aF9 aV9
 E:   .   .  aV9  .  aA9

Defender: move from A2 to B2
Compute time: 0.1 sec
Average depth: 4.3
Heuristic score: -1102

8/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 292, 6: 45338, 2: 43, 1: 8, 5: 55551, 4: 6924}
Average eval depth: 5.3
Average eval time: 0.1
Average branching factor: 5.2
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .   .  aP9
 B:  dT9  .  dF9  .   . 
 C:   .  dP9  .   .   . 
 D:   .  dF9 aP9 aF9 aV9
 E:   .   .  aV9  .  aA9

Attacker: move from D4 to C4
Compute time: 0.0 sec
Average depth: 3.5
Heuristic score: -1456

9/100 moves played
Max search depth: 3
Max search time: 5.0 sec
Total evals at each depth: {3: 339, 6: 45338, 2: 50, 1: 9, 5: 55551, 4: 9396}
Average eval depth: 5.3
Average eval time: 0.1
Average branching factor: 5.2
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .   .  aP9
 B:  dT9  .  dF9  .   . 
 C:   .  dP9  .   .  aV9
 D:   .  dF9 aP9 aF9  . 
 E:   .   .  aV9  .  aA9

Defender: move from B2 to B3
Compute time: 0.0 sec
Average depth: 2.8
Heuristic score: -1102

10/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 812, 6: 45338, 2: 52, 1: 10, 5: 55551, 4: 9396}
Average eval depth: 5.3
Average eval time: 0.1
Average branching factor: 5.2
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .   .  aP9
 B:  dT9  .   .  dF9  . 
 C:   .  dP9  .   .  aV9
 D:   .  dF9 aP9 aF9  . 
 E:   .   .  aV9  .  aA9

Attacker: move from A4 to A3
Compute time: 0.0 sec
Average depth: 3.7
Heuristic score: -1654

11/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 854, 6: 45338, 2: 55, 1: 11, 5: 55551, 4: 12135}
Average eval depth: 5.3
Average eval time: 0.1
Average branching factor: 5.2
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP9  . 
 B:  dT9  .   .  dF9  . 
 C:   .  dP9  .   .  aV9
 D:   .  dF9 aP9 aF9  . 
 E:   .   .  aV9  .  aA9

Defender: move from C1 to C2
Compute time: 0.2 sec
Average depth: 4.5
Heuristic score: -1503

12/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 896, 6: 45338, 2: 61, 1: 12, 5: 76982, 4: 12421}
Average eval depth: 5.2
Average eval time: 0.1
Average branching factor: 5.3
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP9  . 
 B:  dT9  .   .  dF9  . 
 C:   .   .  dP9  .  aV9
 D:   .  dF9 aP9 aF9  . 
 E:   .   .  aV9  .  aA9

Attacker: move from D3 to C3
Compute time: 0.0 sec
Average depth: 3.6
Heuristic score: -1854

13/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 965, 6: 45338, 2: 71, 1: 13, 5: 76982, 4: 17572}
Average eval depth: 5.2
Average eval time: 0.1
Average branching factor: 5.3
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP9  . 
 B:  dT9  .   .  dF9  . 
 C:   .   .  dP9 aF9 aV9
 D:   .  dF9 aP9  .   . 
 E:   .   .  aV9  .  aA9

Defender: move from B0 to B1
Compute time: 0.5 sec
Average depth: 4.5
Heuristic score: -1703

14/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 1043, 6: 45338, 2: 79, 1: 14, 5: 139575, 4: 18215}
Average eval depth: 5.1
Average eval time: 0.1
Average branching factor: 5.6
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP9  . 
 B:   .  dT9  .  dF9  . 
 C:   .   .  dP9 aF9 aV9
 D:   .  dF9 aP9  .   . 
 E:   .   .  aV9  .  aA9

Attacker: move from E2 to E3
Compute time: 0.0 sec
Average depth: 3.6
Heuristic score: -2054

15/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 1118, 6: 45338, 2: 88, 1: 15, 5: 139575, 4: 24263}
Average eval depth: 5.1
Average eval time: 0.1
Average branching factor: 5.6
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP9  . 
 B:   .  dT9  .  dF9  . 
 C:   .   .  dP9 aF9 aV9
 D:   .  dF9 aP9  .   . 
 E:   .   .   .  aV9 aA9

Defender: move from B1 to C1
Compute time: 0.2 sec
Average depth: 4.5
Heuristic score: 1088

16/100 moves played
Max search depth: 6
Max search time: 5.0 sec
Total evals at each depth: {3: 1173, 6: 45338, 2: 94, 1: 16, 5: 171178, 4: 24537}
Average eval depth: 5.1
Average eval time: 0.1
Average branching factor: 5.8
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP9  . 
 B:   .   .   .  dF9  . 
 C:   .  dT9 dP9 aF9 aV9
 D:   .  dF9 aP9  .   . 
 E:   .   .   .  aV9 aA9

Attacker: attack from A3 to B3
combat damage: to source = 1, to target = 1
Compute time: 1.7 sec
Average depth: 5.3
Heuristic score: -5454

17/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 1231, 6: 290606, 2: 101, 1: 17, 5: 173433, 4: 24962}
Average eval depth: 5.5
Average eval time: 0.2
Average branching factor: 6.4
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP8  . 
 B:   .   .   .  dF8  . 
 C:   .  dT9 dP9 aF9 aV9
 D:   .  dF9 aP9  .   . 
 E:   .   .   .  aV9 aA9

Defender: attack from C2 to D2
combat damage: to source = 3, to target = 3
Compute time: 0.2 sec
Average depth: 4.5
Heuristic score: 897

18/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 1279, 6: 290606, 2: 106, 1: 18, 5: 198512, 4: 25161}
Average eval depth: 5.5
Average eval time: 0.2
Average branching factor: 6.4
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP8  . 
 B:   .   .   .  dF8  . 
 C:   .  dT9 dP6 aF9 aV9
 D:   .  dF9 aP6  .   . 
 E:   .   .   .  aV9 aA9

Attacker: move from E3 to E2
Compute time: 0.0 sec
Average depth: 3.6
Heuristic score: -5454

19/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 1334, 6: 290606, 2: 114, 1: 19, 5: 198512, 4: 31787}
Average eval depth: 5.5
Average eval time: 0.2
Average branching factor: 6.4
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP8  . 
 B:   .   .   .  dF8  . 
 C:   .  dT9 dP6 aF9 aV9
 D:   .  dF9 aP6  .   . 
 E:   .   .  aV9  .  aA9

Defender: repair from C1 to C2
repaired 3 health points
Compute time: 0.2 sec
Average depth: 4.5
Heuristic score: 99673

20/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 1388, 6: 290606, 2: 120, 1: 20, 5: 232314, 4: 32026}
Average eval depth: 5.5
Average eval time: 0.2
Average branching factor: 6.5
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP8  . 
 B:   .   .   .  dF8  . 
 C:   .  dT9 dP9 aF9 aV9
 D:   .  dF9 aP6  .   . 
 E:   .   .  aV9  .  aA9

Attacker: move from C4 to B4
Compute time: 0.0 sec
Average depth: 3.6
Heuristic score: -104606

21/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 1431, 6: 290606, 2: 129, 1: 21, 5: 232314, 4: 37537}
Average eval depth: 5.4
Average eval time: 0.2
Average branching factor: 6.5
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP8  . 
 B:   .   .   .  dF8 aV9
 C:   .  dT9 dP9 aF9  . 
 D:   .  dF9 aP6  .   . 
 E:   .   .  aV9  .  aA9

Defender: attack from C2 to D2
combat damage: to source = 3, to target = 3
Compute time: 0.2 sec
Average depth: 4.5
Heuristic score: 102464

22/100 moves played
Max search depth: 6
Max search time: 5.0 sec
Total evals at each depth: {3: 1472, 6: 290606, 2: 133, 1: 22, 5: 255976, 4: 37694}
Average eval depth: 5.4
Average eval time: 0.2
Average branching factor: 6.6
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP8  . 
 B:   .   .   .  dF8 aV9
 C:   .  dT9 dP6 aF9  . 
 D:   .  dF9 aP3  .   . 
 E:   .   .  aV9  .  aA9

Attacker: attack from D2 to D1
combat damage: to source = 1, to target = 1
Compute time: 1.9 sec
Average depth: 5.3
Heuristic score: -110970

23/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 1532, 6: 575538, 2: 139, 1: 23, 5: 258308, 4: 38131}
Average eval depth: 5.6
Average eval time: 0.3
Average branching factor: 6.9
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP8  . 
 B:   .   .   .  dF8 aV9
 C:   .  dT9 dP6 aF9  . 
 D:   .  dF8 aP2  .   . 
 E:   .   .  aV9  .  aA9

Defender: repair from C1 to C2
repaired 3 health points
Compute time: 0.2 sec
Average depth: 4.5
Heuristic score: 105255

24/100 moves played
Max search depth: 6
Max search time: 5.0 sec
Total evals at each depth: {3: 1588, 6: 575538, 2: 145, 1: 24, 5: 293263, 4: 38373}
Average eval depth: 5.6
Average eval time: 0.3
Average branching factor: 6.9
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP8  . 
 B:   .   .   .  dF8 aV9
 C:   .  dT9 dP9 aF9  . 
 D:   .  dF8 aP2  .   . 
 E:   .   .  aV9  .  aA9

Attacker: move from B4 to A4
Compute time: 1.7 sec
Average depth: 5.3
Heuristic score: -112188

25/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 1654, 6: 852654, 2: 153, 1: 25, 5: 295562, 4: 38829}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.1
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP8 aV9
 B:   .   .   .  dF8  . 
 C:   .  dT9 dP9 aF9  . 
 D:   .  dF8 aP2  .   . 
 E:   .   .  aV9  .  aA9

Defender: attack from C2 to D2
combat damage: to source = 3, to target = 3
Compute time: 0.1 sec
Average depth: 4.6
Heuristic score: 106064

26/100 moves played
Max search depth: 6
Max search time: 5.0 sec
Total evals at each depth: {3: 1687, 6: 852654, 2: 155, 1: 26, 5: 313762, 4: 38973}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.1
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP8 aV9
 B:   .   .   .  dF8  . 
 C:   .  dT9 dP6 aF9  . 
 D:   .  dF8  .   .   . 
 E:   .   .  aV9  .  aA9

Attacker: move from E2 to E3
Compute time: 1.1 sec
Average depth: 5.1
Heuristic score: -112392

27/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 1745, 6: 1045226, 2: 163, 1: 27, 5: 315699, 4: 39310}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.1
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP8 aV9
 B:   .   .   .  dF8  . 
 C:   .  dT9 dP6 aF9  . 
 D:   .  dF8  .   .   . 
 E:   .   .   .  aV9 aA9

Defender: repair from C1 to D1
repaired 3 health points
Compute time: 0.2 sec
Average depth: 4.5
Heuristic score: 105866

28/100 moves played
Max search depth: 6
Max search time: 5.0 sec
Total evals at each depth: {3: 1794, 6: 1045226, 2: 170, 1: 28, 5: 343083, 4: 39555}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.1
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP8 aV9
 B:   .   .   .  dF8  . 
 C:   .  dT9 dP6 aF9  . 
 D:   .  dF9  .   .   . 
 E:   .   .   .  aV9 aA9

Attacker: attack from C3 to B3
combat damage: to source = 1, to target = 1
Compute time: 0.6 sec
Average depth: 5.1
Heuristic score: -113588

29/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 1830, 6: 1150766, 2: 176, 1: 29, 5: 344108, 4: 39776}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.2
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP8 aV9
 B:   .   .   .  dF7  . 
 C:   .  dT9 dP6 aF8  . 
 D:   .  dF9  .   .   . 
 E:   .   .   .  aV9 aA9

Defender: attack from C2 to C3
combat damage: to source = 1, to target = 1
Compute time: 0.1 sec
Average depth: 4.4
Heuristic score: 106664

30/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 1873, 6: 1150766, 2: 184, 1: 30, 5: 367560, 4: 39989}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.2
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP8 aV9
 B:   .   .   .  dF7  . 
 C:   .  dT9 dP5 aF7  . 
 D:   .  dF9  .   .   . 
 E:   .   .   .  aV9 aA9

Attacker: move from E3 to E2
Compute time: 0.0 sec
Average depth: 3.6
Heuristic score: -113588

31/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 1909, 6: 1150766, 2: 188, 1: 31, 5: 367560, 4: 44058}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.2
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP8 aV9
 B:   .   .   .  dF7  . 
 C:   .  dT9 dP5 aF7  . 
 D:   .  dF9  .   .   . 
 E:   .   .  aV9  .  aA9

Defender: repair from C1 to C2
repaired 3 health points
Compute time: 0.1 sec
Average depth: 4.5
Heuristic score: 107455

32/100 moves played
Max search depth: 6
Max search time: 5.0 sec
Total evals at each depth: {3: 1937, 6: 1150766, 2: 191, 1: 32, 5: 383145, 4: 44182}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.2
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP8 aV9
 B:   .   .   .  dF7  . 
 C:   .  dT9 dP8 aF7  . 
 D:   .  dF9  .   .   . 
 E:   .   .  aV9  .  aA9

Attacker: attack from A3 to B3
combat damage: to source = 1, to target = 1
Compute time: 0.7 sec
Average depth: 5.2
Heuristic score: -115970

33/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 1976, 6: 1260219, 2: 198, 1: 33, 5: 384271, 4: 44411}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.2
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP7 aV9
 B:   .   .   .  dF6  . 
 C:   .  dT9 dP8 aF7  . 
 D:   .  dF9  .   .   . 
 E:   .   .  aV9  .  aA9

Defender: attack from C2 to C3
combat damage: to source = 1, to target = 1
Compute time: 0.1 sec
Average depth: 4.5
Heuristic score: 108255

34/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 2012, 6: 1260219, 2: 202, 1: 34, 5: 401644, 4: 44536}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.2
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP7 aV9
 B:   .   .   .  dF6  . 
 C:   .  dT9 dP7 aF6  . 
 D:   .  dF9  .   .   . 
 E:   .   .  aV9  .  aA9

Attacker: attack from C3 to B3
combat damage: to source = 1, to target = 1
Compute time: 0.0 sec
Average depth: 3.6
Heuristic score: -115970

35/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 2050, 6: 1260219, 2: 208, 1: 35, 5: 401644, 4: 48620}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.2
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP7 aV9
 B:   .   .   .  dF5  . 
 C:   .  dT9 dP7 aF5  . 
 D:   .  dF9  .   .   . 
 E:   .   .  aV9  .  aA9

Defender: attack from C2 to C3
combat damage: to source = 1, to target = 1
Compute time: 0.1 sec
Average depth: 4.5
Heuristic score: 108066

36/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 2094, 6: 1260219, 2: 212, 1: 36, 5: 423401, 4: 48784}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.2
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP7 aV9
 B:   .   .   .  dF5  . 
 C:   .  dT9 dP6 aF4  . 
 D:   .  dF9  .   .   . 
 E:   .   .  aV9  .  aA9

Attacker: move from E2 to D2
Compute time: 0.0 sec
Average depth: 3.6
Heuristic score: -116192

37/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 2154, 6: 1260219, 2: 221, 1: 37, 5: 423401, 4: 55193}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.2
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP7 aV9
 B:   .   .   .  dF5  . 
 C:   .  dT9 dP6 aF4  . 
 D:   .  dF9 aV9  .   . 
 E:   .   .   .   .  aA9

Defender: repair from C1 to C2
repaired 3 health points
Compute time: 0.1 sec
Average depth: 4.5
Heuristic score: 108864

38/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 2202, 6: 1260219, 2: 227, 1: 38, 5: 449003, 4: 55463}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.2
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP7 aV9
 B:   .   .   .  dF5  . 
 C:   .  dT9 dP9 aF4  . 
 D:   .  dF9 aV9  .   . 
 E:   .   .   .   .  aA9

Attacker: move from D2 to E2
Compute time: 0.0 sec
Average depth: 3.7
Heuristic score: -117388

39/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 2244, 6: 1260219, 2: 231, 1: 39, 5: 449003, 4: 58615}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.2
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP7 aV9
 B:   .   .   .  dF5  . 
 C:   .  dT9 dP9 aF4  . 
 D:   .  dF9  .   .   . 
 E:   .   .  aV9  .  aA9

Defender: attack from B3 to C3
combat damage: to source = 1, to target = 1
Compute time: 0.2 sec
Average depth: 4.5
Heuristic score: 111649

40/100 moves played
Max search depth: 6
Max search time: 5.0 sec
Total evals at each depth: {3: 2293, 6: 1260219, 2: 237, 1: 40, 5: 476460, 4: 58870}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.2
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP7 aV9
 B:   .   .   .  dF4  . 
 C:   .  dT9 dP9 aF3  . 
 D:   .  dF9  .   .   . 
 E:   .   .  aV9  .  aA9

Attacker: attack from A3 to B3
combat damage: to source = 1, to target = 1
Compute time: 0.6 sec
Average depth: 5.1
Heuristic score: -121764

41/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 2334, 6: 1363456, 2: 242, 1: 41, 5: 477742, 4: 59080}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.1
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP6 aV9
 B:   .   .   .  dF3  . 
 C:   .  dT9 dP9 aF3  . 
 D:   .  dF9  .   .   . 
 E:   .   .  aV9  .  aA9

Defender: attack from C2 to C3
combat damage: to source = 1, to target = 1
Compute time: 0.1 sec
Average depth: 4.5
Heuristic score: 112458

42/100 moves played
Max search depth: 6
Max search time: 5.0 sec
Total evals at each depth: {3: 2371, 6: 1363456, 2: 245, 1: 42, 5: 497499, 4: 59281}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.2
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP6 aV9
 B:   .   .   .  dF3  . 
 C:   .  dT9 dP8 aF2  . 
 D:   .  dF9  .   .   . 
 E:   .   .  aV9  .  aA9

Attacker: self-destruct at C3
self-destructed for 4 total damage
Compute time: 0.5 sec
Average depth: 5.0
Heuristic score: -121958

43/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 2415, 6: 1450697, 2: 251, 1: 43, 5: 498821, 4: 59468}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.1
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  aP6 aV9
 B:   .   .   .  dF1  . 
 C:   .  dT9 dP6  .   . 
 D:   .  dF9  .   .   . 
 E:   .   .  aV9  .  aA9

Defender: self-destruct at B3
self-destructed for 6 total damage
Compute time: 0.1 sec
Average depth: 4.3
Heuristic score: 112255

44/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 2453, 6: 1450697, 2: 258, 1: 44, 5: 513287, 4: 59639}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.1
Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  aP4 aV7
 B:   .   .   .   .   . 
 C:   .  dT9 dP4  .   . 
 D:   .  dF9  .   .   . 
 E:   .   .  aV9  .  aA9

Attacker: move from A3 to A2
Compute time: 0.0 sec
Average depth: 3.6
Heuristic score: -121958

45/100 moves played
Max search depth: 3
Max search time: 5.0 sec
Total evals at each depth: {3: 2488, 6: 1450697, 2: 262, 1: 45, 5: 513287, 4: 62206}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.1
Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9 aP4  .  aV7
 B:   .   .   .   .   . 
 C:   .  dT9 dP4  .   . 
 D:   .  dF9  .   .   . 
 E:   .   .  aV9  .  aA9

Defender: move from A1 to B1
Compute time: 0.0 sec
Average depth: 2.8
Heuristic score: 112255

46/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 3053, 6: 1450697, 2: 268, 1: 46, 5: 513287, 4: 62206}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.1
Next player: Attacker

      0   1   2   3   4 
 A:  dA9  .  aP4  .  aV7
 B:   .  dT9  .   .   . 
 C:   .  dT9 dP4  .   . 
 D:   .  dF9  .   .   . 
 E:   .   .  aV9  .  aA9

Attacker: move from E2 to E1
Compute time: 0.0 sec
Average depth: 3.6
Heuristic score: -124140

47/100 moves played
Max search depth: 5
Max search time: 5.0 sec
Total evals at each depth: {3: 3093, 6: 1450697, 2: 274, 1: 47, 5: 513287, 4: 65758}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.1
Next player: Defender

      0   1   2   3   4 
 A:  dA9  .  aP4  .  aV7
 B:   .  dT9  .   .   . 
 C:   .  dT9 dP4  .   . 
 D:   .  dF9  .   .   . 
 E:   .  aV9  .   .  aA9

Defender: move from A0 to A1
Compute time: 0.1 sec
Average depth: 4.4
Heuristic score: 182830

48/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 3125, 6: 1450697, 2: 280, 1: 48, 5: 526223, 4: 65906}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.1
Next player: Attacker

      0   1   2   3   4 
 A:   .  dA9 aP4  .  aV7
 B:   .  dT9  .   .   . 
 C:   .  dT9 dP4  .   . 
 D:   .  dF9  .   .   . 
 E:   .  aV9  .   .  aA9

Attacker: move from E4 to E3
Compute time: 0.0 sec
Average depth: 3.6
Heuristic score: -193296

49/100 moves played
Max search depth: 3
Max search time: 5.0 sec
Total evals at each depth: {3: 3161, 6: 1450697, 2: 287, 1: 49, 5: 526223, 4: 68964}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.1
Next player: Defender

      0   1   2   3   4 
 A:   .  dA9 aP4  .  aV7
 B:   .  dT9  .   .   . 
 C:   .  dT9 dP4  .   . 
 D:   .  dF9  .   .   . 
 E:   .  aV9  .  aA9  . 

Defender: attack from A1 to A2
combat damage: to source = 3, to target = 3
Compute time: 0.0 sec
Average depth: 2.7
Heuristic score: 182830

50/100 moves played
Max search depth: 2
Max search time: 5.0 sec
Total evals at each depth: {3: 3653, 6: 1450697, 2: 295, 1: 50, 5: 526223, 4: 68964}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.1
Next player: Attacker

      0   1   2   3   4 
 A:   .  dA6 aP1  .  aV7
 B:   .  dT9  .   .   . 
 C:   .  dT9 dP4  .   . 
 D:   .  dF9  .   .   . 
 E:   .  aV9  .  aA9  . 

Attacker: move from A4 to A3
Compute time: 0.0 sec
Average depth: 1.9
Heuristic score: -193296

51/100 moves played
Max search depth: 3
Max search time: 5.0 sec
Total evals at each depth: {3: 3653, 6: 1450697, 2: 405, 1: 51, 5: 526223, 4: 68964}
Average eval depth: 5.7
Average eval time: 0.3
Average branching factor: 7.1
Next player: Defender

      0   1   2   3   4 
 A:   .  dA6 aP1 aV7  . 
 B:   .  dT9  .   .   . 
 C:   .  dT9 dP4  .   . 
 D:   .  dF9  .   .   . 
 E:   .  aV9  .  aA9  . 

Defender: repair from B1 to A1
repaired 3 health points
Compute time: 0.0 sec
Average depth: 2.8
Heuristic score: 197638

52/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 3990, 6: 1450697, 2: 408, 1: 52, 5: 526223, 4: 68964}
Average eval depth: 5.7
Average eval time: 0.2
Average branching factor: 7.1
Next player: Attacker

      0   1   2   3   4 
 A:   .  dA9 aP1 aV7  . 
 B:   .  dT9  .   .   . 
 C:   .  dT9 dP4  .   . 
 D:   .  dF9  .   .   . 
 E:   .  aV9  .  aA9  . 

Attacker: move from E3 to E2
Compute time: 0.0 sec
Average depth: 3.6
Heuristic score: -223682

53/100 moves played
Max search depth: 3
Max search time: 5.0 sec
Total evals at each depth: {3: 4032, 6: 1450697, 2: 415, 1: 53, 5: 526223, 4: 72365}
Average eval depth: 5.7
Average eval time: 0.2
Average branching factor: 7.1
Next player: Defender

      0   1   2   3   4 
 A:   .  dA9 aP1 aV7  . 
 B:   .  dT9  .   .   . 
 C:   .  dT9 dP4  .   . 
 D:   .  dF9  .   .   . 
 E:   .  aV9 aA9  .   . 

Defender: attack from A1 to A2
combat damage: to source = 3, to target = 3
Compute time: 0.0 sec
Average depth: 2.8
Heuristic score: 212429

54/100 moves played
Max search depth: 4
Max search time: 5.0 sec
Total evals at each depth: {3: 4266, 6: 1450697, 2: 417, 1: 54, 5: 526223, 4: 72365}
Average eval depth: 5.7
Average eval time: 0.2
Average branching factor: 7.1
Next player: Attacker

      0   1   2   3   4 
 A:   .  dA6  .  aV7  . 
 B:   .  dT9  .   .   . 
 C:   .  dT9 dP4  .   . 
 D:   .  dF9  .   .   . 
 E:   .  aV9 aA9  .   . 

Attacker: move from A3 to A2
Compute time: 0.0 sec
Average depth: 3.4
Heuristic score: 2147483590

55/100 moves played
Max search depth: 3
Max search time: 5.0 sec
Total evals at each depth: {3: 4304, 6: 1450697, 2: 422, 1: 55, 5: 526223, 4: 73749}
Average eval depth: 5.7
Average eval time: 0.2
Average branching factor: 7.1
Next player: Defender

      0   1   2   3   4 
 A:   .  dA6 aV7  .   . 
 B:   .  dT9  .   .   . 
 C:   .  dT9 dP4  .   . 
 D:   .  dF9  .   .   . 
 E:   .  aV9 aA9  .   . 

Defender: repair from C1 to C2
repaired 3 health points
Compute time: 0.0 sec
Average depth: 2.5
Heuristic score: -2147483591

56/100 moves played
Max search depth: 2
Max search time: 5.0 sec
Total evals at each depth: {3: 4826, 6: 1450697, 2: 446, 1: 57, 5: 526223, 4: 73749}
Average eval depth: 5.7
Average eval time: 0.2
Average branching factor: 7.1
Next player: Attacker

      0   1   2   3   4 
 A:   .  dA6 aV7  .   . 
 B:   .  dT9  .   .   . 
 C:   .  dT9 dP7  .   . 
 D:   .  dF9  .   .   . 
 E:   .  aV9 aA9  .   . 

Attacker: attack from A2 to A1
combat damage: to source = 3, to target = 9
Compute time: 0.0 sec
Average depth: 1.8
Heuristic score: 2147483590

57/100 moves played
Max search depth: 1
Max search time: 5.0 sec
Total evals at each depth: {3: 4826, 6: 1450697, 2: 509, 1: 59, 5: 526223, 4: 73749}
Average eval depth: 5.7
Average eval time: 0.2
Average branching factor: 7.1
Next player: Defender

      0   1   2   3   4 
 A:   .   .  aV4  .   . 
 B:   .  dT9  .   .   . 
 C:   .  dT9 dP7  .   . 
 D:   .  dF9  .   .   . 
 E:   .  aV9 aA9  .   . 

Attacker wins in 57 moves!
```