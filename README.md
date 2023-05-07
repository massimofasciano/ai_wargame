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

### Short trace with full debug (end of an AI vs AI game)

Attacker starts seeing victory in heuristic score at move 50 of 57 (search depth = 7).

```
47/100 moves played
Max search depth: 7
Max search time: 5.0 sec
Total evals at each depth: {5: 53511, 2: 276, 1: 48, 4: 10038, 3: 1821, 6: 292984, 7: 22189931}
Average eval depth: 7.0
Average eval time: 1.2
Average branching factor: 6.4
Next player: Defender

      0   1   2   3   4
 A:   .   .   .   .  dT1
 B:  dA4  .   .  aF9  .
 C:  dT6  .   .  aV9 aA9
 D:   .   .   .   .   .
 E:  dF9  .   .   .   .

Defender: repair from C0 to B0
repaired 3 health points
Compute time: 0.2 sec
Average depth: 5.5
Heuristic score: -5381

48/100 moves played
Max search depth: 7
Max search time: 5.0 sec
Total evals at each depth: {5: 53940, 2: 280, 1: 49, 4: 10130, 3: 1844, 6: 294738, 7: 22252296}
Average eval depth: 7.0
Average eval time: 1.2
Average branching factor: 6.4
Next player: Attacker

      0   1   2   3   4
 A:   .   .   .   .  dT1
 B:  dA7  .   .  aF9  .
 C:  dT6  .   .  aV9 aA9
 D:   .   .   .   .   .
 E:  dF9  .   .   .   .

Attacker: move from C3 to C2
Compute time: 0.1 sec
Average depth: 4.8
Heuristic score: 3750

49/100 moves played
Max search depth: 7
Max search time: 5.0 sec
Total evals at each depth: {5: 54301, 2: 284, 1: 50, 4: 10195, 3: 1861, 6: 296303, 7: 22312178}
Average eval depth: 7.0
Average eval time: 1.2
Average branching factor: 6.4
Next player: Defender

      0   1   2   3   4
 A:   .   .   .   .  dT1
 B:  dA7  .   .  aF9  .
 C:  dT6  .  aV9  .  aA9
 D:   .   .   .   .   .
 E:  dF9  .   .   .   .

Defender: move from C0 to C1
Compute time: 0.3 sec
Average depth: 5.3
Heuristic score: -268077

50/100 moves played
Max search depth: 7
Max search time: 5.0 sec
Total evals at each depth: {5: 55398, 2: 292, 1: 51, 4: 10458, 3: 1906, 6: 302536, 7: 22420688}
Average eval depth: 7.0
Average eval time: 1.1
Average branching factor: 6.4
Next player: Attacker

      0   1   2   3   4
 A:   .   .   .   .  dT1
 B:  dA7  .   .  aF9  .
 C:   .  dT6 aV9  .  aA9
 D:   .   .   .   .   .
 E:  dF9  .   .   .   .

Attacker: attack from C2 to C1
combat damage: to source = 6, to target = 6
Compute time: 0.1 sec
Average depth: 5.4
Heuristic score: 2147483590

51/100 moves played
Max search depth: 7
Max search time: 5.0 sec
Total evals at each depth: {5: 56212, 2: 296, 1: 52, 4: 10584, 3: 1939, 6: 305326, 7: 22521100}
Average eval depth: 7.0
Average eval time: 1.1
Average branching factor: 6.4
Next player: Defender

      0   1   2   3   4
 A:   .   .   .   .  dT1
 B:  dA7  .   .  aF9  .
 C:   .   .  aV3  .  aA9
 D:   .   .   .   .   .
 E:  dF9  .   .   .   .

Defender: self-destruct at A4
self-destructed for 2 total damage
Compute time: 0.1 sec
Average depth: 4.8
Heuristic score: -2147483591

52/100 moves played
Max search depth: 7
Max search time: 5.0 sec
Total evals at each depth: {5: 57006, 2: 302, 1: 53, 4: 10782, 3: 1976, 6: 308483, 7: 22548835}
Average eval depth: 7.0
Average eval time: 1.1
Average branching factor: 6.3
Next player: Attacker

      0   1   2   3   4
 A:   .   .   .   .   .
 B:  dA7  .   .  aF7  .
 C:   .   .  aV3  .  aA9
 D:   .   .   .   .   .
 E:  dF9  .   .   .   .

Attacker: move from C2 to C1
Compute time: 0.0 sec
Average depth: 4.7
Heuristic score: 2147483590

53/100 moves played
Max search depth: 7
Max search time: 5.0 sec
Total evals at each depth: {5: 57303, 2: 308, 1: 54, 4: 10878, 3: 1994, 6: 309321, 7: 22563520}
Average eval depth: 7.0
Average eval time: 1.1
Average branching factor: 6.3
Next player: Defender

      0   1   2   3   4
 A:   .   .   .   .   .
 B:  dA7  .   .  aF7  .
 C:   .  aV3  .   .  aA9
 D:   .   .   .   .   .
 E:  dF9  .   .   .   .

Defender: move from E0 to E1
Compute time: 0.0 sec
Average depth: 3.8
Heuristic score: -2147483591

54/100 moves played
Max search depth: 7
Max search time: 5.0 sec
Total evals at each depth: {5: 57582, 2: 314, 1: 55, 4: 10962, 3: 2019, 6: 309930, 7: 22567385}
Average eval depth: 7.0
Average eval time: 1.1
Average branching factor: 6.3
Next player: Attacker

      0   1   2   3   4
 A:   .   .   .   .   .
 B:  dA7  .   .  aF7  .
 C:   .  aV3  .   .  aA9
 D:   .   .   .   .   .
 E:   .  dF9  .   .   .

Attacker: move from C1 to B1
Compute time: 0.0 sec
Average depth: 4.4
Heuristic score: 2147483590

55/100 moves played
Max search depth: 7
Max search time: 5.0 sec
Total evals at each depth: {5: 57923, 2: 323, 1: 56, 4: 11089, 3: 2057, 6: 310609, 7: 22576101}
Average eval depth: 7.0
Average eval time: 1.0
Average branching factor: 6.3
Next player: Defender

      0   1   2   3   4
 A:   .   .   .   .   .
 B:  dA7 aV3  .  aF7  .
 C:   .   .   .   .  aA9
 D:   .   .   .   .   .
 E:   .  dF9  .   .   .

Defender: self-destruct at E1
self-destructed for 0 total damage
Compute time: 0.0 sec
Average depth: 2.5
Heuristic score: -2147483591

56/100 moves played
Max search depth: 7
Max search time: 5.0 sec
Total evals at each depth: {5: 58007, 2: 327, 1: 58, 4: 11125, 3: 2080, 6: 310774, 7: 22577324}
Average eval depth: 7.0
Average eval time: 1.0
Average branching factor: 6.3
Next player: Attacker

      0   1   2   3   4
 A:   .   .   .   .   .
 B:  dA7 aV3  .  aF7  .
 C:   .   .   .   .  aA9
 D:   .   .   .   .   .
 E:   .   .   .   .   .

Attacker: attack from B1 to B0
combat damage: to source = 3, to target = 9
Compute time: 0.0 sec
Average depth: 2.9
Heuristic score: 2147483590

57/100 moves played
Max search depth: 7
Max search time: 5.0 sec
Total evals at each depth: {5: 58026, 2: 336, 1: 60, 4: 11139, 3: 2086, 6: 310826, 7: 22577707}
Average eval depth: 7.0
Average eval time: 1.0
Average branching factor: 6.3
Next player: Defender

      0   1   2   3   4
 A:   .   .   .   .   .
 B:   .   .   .  aF7  .
 C:   .   .   .   .  aA9
 D:   .   .   .   .   .
 E:   .   .   .   .   .

Attacker wins in 57 moves!
```

### Example output (full AI vs AI game)

At move 39, Defender is stuck and sees defeat so it starts performing pointless self-destructs to delay the inevitable.

```
0/100 moves played

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9 dP9  .   .   . 
 C:  dF9  .   .   .  aP9
 D:   .   .   .  aF9 aV9
 E:   .   .  aP9 aV9 aA9

Attacker: move from E2 to D2

1/100 moves played

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9 dP9  .   .   . 
 C:  dF9  .   .   .  aP9
 D:   .   .  aP9 aF9 aV9
 E:   .   .   .  aV9 aA9

Defender: move from B1 to B2

2/100 moves played

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9  .  dP9  .   . 
 C:  dF9  .   .   .  aP9
 D:   .   .  aP9 aF9 aV9
 E:   .   .   .  aV9 aA9

Attacker: move from D2 to D1

3/100 moves played

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9  .  dP9  .   . 
 C:  dF9  .   .   .  aP9
 D:   .  aP9  .  aF9 aV9
 E:   .   .   .  aV9 aA9

Defender: move from C0 to D0

4/100 moves played

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9  .  dP9  .   . 
 C:   .   .   .   .  aP9
 D:  dF9 aP9  .  aF9 aV9
 E:   .   .   .  aV9 aA9

Attacker: move from C4 to C3

5/100 moves played

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9  .  dP9  .   . 
 C:   .   .   .  aP9  . 
 D:  dF9 aP9  .  aF9 aV9
 E:   .   .   .  aV9 aA9

Defender: attack from D0 to D1
combat damage: to source = 1, to target = 1

6/100 moves played

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9  .  dP9  .   . 
 C:   .   .   .  aP9  . 
 D:  dF8 aP8  .  aF9 aV9
 E:   .   .   .  aV9 aA9

Attacker: move from D4 to C4

7/100 moves played

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9  .  dP9  .   . 
 C:   .   .   .  aP9 aV9
 D:  dF8 aP8  .  aF9  . 
 E:   .   .   .  aV9 aA9

Defender: move from B0 to C0

8/100 moves played

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:   .   .  dP9  .   . 
 C:  dT9  .   .  aP9 aV9
 D:  dF8 aP8  .  aF9  . 
 E:   .   .   .  aV9 aA9

Attacker: move from C4 to B4

9/100 moves played

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:   .   .  dP9  .  aV9
 C:  dT9  .   .  aP9  . 
 D:  dF8 aP8  .  aF9  . 
 E:   .   .   .  aV9 aA9

Defender: move from B2 to C2

10/100 moves played

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:   .   .   .   .  aV9
 C:  dT9  .  dP9 aP9  . 
 D:  dF8 aP8  .  aF9  . 
 E:   .   .   .  aV9 aA9

Attacker: move from E3 to E2

11/100 moves played

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:   .   .   .   .  aV9
 C:  dT9  .  dP9 aP9  . 
 D:  dF8 aP8  .  aF9  . 
 E:   .   .  aV9  .  aA9

Defender: move from A1 to B1

12/100 moves played

      0   1   2   3   4 
 A:  dA9  .  dF9  .   . 
 B:   .  dT9  .   .  aV9
 C:  dT9  .  dP9 aP9  . 
 D:  dF8 aP8  .  aF9  . 
 E:   .   .  aV9  .  aA9

Attacker: move from E4 to E3

13/100 moves played

      0   1   2   3   4 
 A:  dA9  .  dF9  .   . 
 B:   .  dT9  .   .  aV9
 C:  dT9  .  dP9 aP9  . 
 D:  dF8 aP8  .  aF9  . 
 E:   .   .  aV9 aA9  . 

Defender: move from A0 to B0

14/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:  dA9 dT9  .   .  aV9
 C:  dT9  .  dP9 aP9  . 
 D:  dF8 aP8  .  aF9  . 
 E:   .   .  aV9 aA9  . 

Attacker: move from B4 to C4

15/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:  dA9 dT9  .   .   . 
 C:  dT9  .  dP9 aP9 aV9
 D:  dF8 aP8  .  aF9  . 
 E:   .   .  aV9 aA9  . 

Defender: move from B1 to B2

16/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:  dA9  .  dT9  .   . 
 C:  dT9  .  dP9 aP9 aV9
 D:  dF8 aP8  .  aF9  . 
 E:   .   .  aV9 aA9  . 

Attacker: move from E2 to D2

17/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:  dA9  .  dT9  .   . 
 C:  dT9  .  dP9 aP9 aV9
 D:  dF8 aP8 aV9 aF9  . 
 E:   .   .   .  aA9  . 

Defender: attack from D0 to D1
combat damage: to source = 1, to target = 1

18/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:  dA9  .  dT9  .   . 
 C:  dT9  .  dP9 aP9 aV9
 D:  dF7 aP7 aV9 aF9  . 
 E:   .   .   .  aA9  . 

Attacker: attack from D2 to C2
combat damage: to source = 3, to target = 6

19/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:  dA9  .  dT9  .   . 
 C:  dT9  .  dP3 aP9 aV9
 D:  dF7 aP7 aV6 aF9  . 
 E:   .   .   .  aA9  . 

Defender: repair from B2 to C2
repaired 3 health points

20/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:  dA9  .  dT9  .   . 
 C:  dT9  .  dP6 aP9 aV9
 D:  dF7 aP7 aV6 aF9  . 
 E:   .   .   .  aA9  . 

Attacker: attack from D2 to C2
combat damage: to source = 3, to target = 6

21/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:  dA9  .  dT9  .   . 
 C:  dT9  .   .  aP9 aV9
 D:  dF7 aP7 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Defender: attack from D0 to D1
combat damage: to source = 1, to target = 1

22/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:  dA9  .  dT9  .   . 
 C:  dT9  .   .  aP9 aV9
 D:  dF6 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Attacker: move from C3 to C2

23/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:  dA9  .  dT9  .   . 
 C:  dT9  .  aP9  .  aV9
 D:  dF6 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Defender: move from B2 to B1

24/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:  dA9 dT9  .   .   . 
 C:  dT9  .  aP9  .  aV9
 D:  dF6 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Attacker: move from C2 to C1

25/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:  dA9 dT9  .   .   . 
 C:  dT9 aP9  .   .  aV9
 D:  dF6 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Defender: repair from C0 to D0
repaired 3 health points

26/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:  dA9 dT9  .   .   . 
 C:  dT9 aP9  .   .  aV9
 D:  dF9 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Attacker: attack from C1 to C0
combat damage: to source = 1, to target = 3

27/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:  dA9 dT9  .   .   . 
 C:  dT6 aP8  .   .  aV9
 D:  dF9 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Defender: move from B1 to B2

28/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:  dA9  .  dT9  .   . 
 C:  dT6 aP8  .   .  aV9
 D:  dF9 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Attacker: attack from C1 to C0
combat damage: to source = 1, to target = 3

29/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:  dA9  .  dT9  .   . 
 C:  dT3 aP7  .   .  aV9
 D:  dF9 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Defender: move from B0 to B1

30/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:   .  dA9 dT9  .   . 
 C:  dT3 aP7  .   .  aV9
 D:  dF9 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Attacker: attack from C1 to C0
combat damage: to source = 1, to target = 3

31/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:   .  dA9 dT9  .   . 
 C:   .  aP6  .   .  aV9
 D:  dF9 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Defender: attack from B1 to C1
combat damage: to source = 3, to target = 3

32/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:   .  dA6 dT9  .   . 
 C:   .  aP3  .   .  aV9
 D:  dF9 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Attacker: move from C4 to C3

33/100 moves played

      0   1   2   3   4 
 A:   .   .  dF9  .   . 
 B:   .  dA6 dT9  .   . 
 C:   .  aP3  .  aV9  . 
 D:  dF9 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Defender: move from A2 to A3

34/100 moves played

      0   1   2   3   4 
 A:   .   .   .  dF9  . 
 B:   .  dA6 dT9  .   . 
 C:   .  aP3  .  aV9  . 
 D:  dF9 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Attacker: move from C3 to C2

35/100 moves played

      0   1   2   3   4 
 A:   .   .   .  dF9  . 
 B:   .  dA6 dT9  .   . 
 C:   .  aP3 aV9  .   . 
 D:  dF9 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Defender: attack from B2 to C2
combat damage: to source = 6, to target = 6

36/100 moves played

      0   1   2   3   4 
 A:   .   .   .  dF9  . 
 B:   .  dA6 dT3  .   . 
 C:   .  aP3 aV3  .   . 
 D:  dF9 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Attacker: attack from C1 to B1
combat damage: to source = 3, to target = 3

37/100 moves played

      0   1   2   3   4 
 A:   .   .   .  dF9  . 
 B:   .  dA3 dT3  .   . 
 C:   .   .  aV3  .   . 
 D:  dF9 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Defender: attack from B2 to C2
combat damage: to source = 6, to target = 6

38/100 moves played

      0   1   2   3   4 
 A:   .   .   .  dF9  . 
 B:   .  dA3  .   .   . 
 C:   .   .   .   .   . 
 D:  dF9 aP6 aV3 aF9  . 
 E:   .   .   .  aA9  . 

Attacker: move from D2 to C2

39/100 moves played

      0   1   2   3   4 
 A:   .   .   .  dF9  . 
 B:   .  dA3  .   .   . 
 C:   .   .  aV3  .   . 
 D:  dF9 aP6  .  aF9  . 
 E:   .   .   .  aA9  . 

Defender: self-destruct at A3
self-destructed for 0 total damage

40/100 moves played

      0   1   2   3   4 
 A:   .   .   .   .   . 
 B:   .  dA3  .   .   . 
 C:   .   .  aV3  .   . 
 D:  dF9 aP6  .  aF9  . 
 E:   .   .   .  aA9  . 

Attacker: move from C2 to B2

41/100 moves played

      0   1   2   3   4 
 A:   .   .   .   .   . 
 B:   .  dA3 aV3  .   . 
 C:   .   .   .   .   . 
 D:  dF9 aP6  .  aF9  . 
 E:   .   .   .  aA9  . 

Defender: self-destruct at D0
self-destructed for 2 total damage

42/100 moves played

      0   1   2   3   4 
 A:   .   .   .   .   . 
 B:   .  dA3 aV3  .   . 
 C:   .   .   .   .   . 
 D:   .  aP4  .  aF9  . 
 E:   .   .   .  aA9  . 

Attacker: attack from B2 to B1
combat damage: to source = 3, to target = 9

43/100 moves played

      0   1   2   3   4 
 A:   .   .   .   .   . 
 B:   .   .   .   .   . 
 C:   .   .   .   .   . 
 D:   .  aP4  .  aF9  . 
 E:   .   .   .  aA9  . 

Attacker wins in 43 moves!
```