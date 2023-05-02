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

Movement and other actions cannot be diagonal (only left, top, right, bottom).

Attacker always moves up or left.
Defender always moves down or right.
Exception: Tech and Virus can always move in any direction.
Actual movement is possible only if destination is empty.

A unit can also attack or repair another adjacent unit (in any of the 4 directions) instead of moving.
Refer to damage and repair tables for results.
In order for the attack or repair action to be valid, it must result in a change of health on either side.
All units start with 9 points of health and are removed after combat if their health is 0.
Health cannot go above 9 or below 0.
When combat occurs between opposing units (X attacks Y), X damages Y but Y also damages X according to the same damage tables (mutual combat).
When repair happens between friendly units, the initiator repairs the target (not bi-directional).

Units are said to be engaged in combat if an opposing unit is adjacent (in any of the 4 directions).
A unit cannot move if engaged in combat, except the Virus and the Tech.

The game ends when any of these conditions is true:

- a player loses their AI
- a player cannot perform any action
- a pre-determined number of moves has been reached (150 is a good number)

To determine the winner at the end of the game, these rules are applied:

- a player wins if their AI is alive when the other AI is destroyed
- a score is calculated based on the remaining units (see scoring table). Highest wins.
- the defender wins if the scores are equal (because attacker starts first)

## Tables

### Unit values for scoring
```
AI => 50,
Virus => 25,
Tech => 25,
Firewall => 10,
Program => 10,
```

### Damage tables for attacker to defender
```
AI => 
    Firewall => 1,
    _ => 3,
Virus => 
    AI => 9,
    Tech | Program => 6,
    Virus | Firewall => 1,
Tech => 
    Virus => 6,
    _ => 1,
Firewall => 
    _ => 1,
Program => 
    Firewall => 1,
    _ => 3,
```
A few important details:

- the Virus can destroy the AI in 1 attack (9 points)
- the Firewall is good at absorbing attacks but bad at damaging other units
- Tech and Virus are equal in combat against each other
- Virus is very offensive
- Tech is very defensive

### Repair tables for friendly units
```
Tech => 
    AI | Firewall | Program => 3,
    _ => 0,
AI  => 
    Virus | Tech => 1,
    _ => 0,
_ => 0,
```
As you can see, the Tech can repair AI, Firewall and Program by 3 points.
The AI can repair the Virus and Tech by 1 point.
These are the only allowed repairs.

### Common strategies

- A Tech unit behind AI, Firewall, Program can make them invincible except to a Virus.
- A Virus can move back to it's AI to get slowly repaired
- A Tech can move around in any direction to help units in need of repair
- The AI cannot move back so it can be blocked by some units and then destroyed by a Virus
- Victory by point score happens quite often: placing one's own units in a situation where they can't move can end the game favorably.

### Heuristics

- The 2 players have different sets of units so using specialized heuristics for each one will help.

### Example output (AI vs AI with full debug traces, manually shortened)

```
# 0/150 moves played
# Max search depth: 6
# Max search time: 5.0 sec
# Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9 dP9  .   .   . 
 C:  dF9  .   .   .  aP9
 D:   .   .   .  aF9 aV9
 E:   .   .  aP9 aV9 aA9

-> Attacker: move from E2 to D2
-> Defender: move from B1 to C1
-> Attacker: move from E3 to E2
-> Defender: move from C0 to D0
-> Attacker: move from E2 to E3
-> Defender: move from C1 to D1
-> Attacker: move from E3 to E2
-> Defender: move from D0 to E0
-> Attacker: move from E2 to E1

# 9/150 moves played
# Max search depth: 8
# Max search time: 5.0 sec
# Total evals at each depth: {6: 4151, 7: 2561856, 8: 1705514}
# Average eval depth: 7.4
# Average eval time: 1.1
# Average branching factor: 6.5
# Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9  .   .   .   . 
 C:   .   .   .   .  aP9
 D:   .  dP9 aP9 aF9 aV9
 E:  dF9 aV9  .   .  aA9

-> Defender: attack from E0 to E1
# combat damage: to source = 1, to target = 1
# Compute time: 2.4 sec
# Average depth: 8.0
# Heuristic score: -967

# 10/150 moves played
# Max search depth: 8
# Max search time: 5.0 sec
# Total evals at each depth: {6: 4151, 7: 2561856, 8: 3082575}
# Average eval depth: 7.5
# Average eval time: 1.2
# Average branching factor: 6.2
# Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9  .   .   .   . 
 C:   .   .   .   .  aP9
 D:   .  dP9 aP9 aF9 aV9
 E:  dF8 aV8  .   .  aA9

-> Attacker: attack from E1 to D1
# combat damage: to source = 3, to target = 6
# Compute time: 1.8 sec
# Average depth: 8.0
# Heuristic score: 93160

# 11/150 moves played
# Max search depth: 8
# Max search time: 5.0 sec
# Total evals at each depth: {6: 4151, 7: 2561856, 8: 3295618}
# Average eval depth: 7.6
# Average eval time: 1.3
# Average branching factor: 5.9
# Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9  .   .   .   . 
 C:   .   .   .   .  aP9
 D:   .  dP3 aP9 aF9 aV9
 E:  dF8 aV5  .   .  aA9

-> Defender: attack from D1 to E1
# combat damage: to source = 6, to target = 3
# Compute time: 0.7 sec
# Average depth: 8.0
# Heuristic score: -979

# 12/150 moves played
# Max search depth: 8
# Max search time: 5.0 sec
# Total evals at each depth: {6: 4151, 7: 2561857, 8: 3740678}
# Average eval depth: 7.6
# Average eval time: 1.2
# Average branching factor: 5.9
# Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9  .   .   .   . 
 C:   .   .   .   .  aP9
 D:   .   .  aP9 aF9 aV9
 E:  dF8 aV2  .   .  aA9

-> Attacker: move from E1 to E2
-> Defender: move from E0 to E1
-> Attacker: move from D2 to D1

# 15/150 moves played
# Max search depth: 8
# Max search time: 5.0 sec
# Total evals at each depth: {6: 4151, 7: 2561858, 8: 4577503}
# Average eval depth: 7.6
# Average eval time: 1.3
# Average branching factor: 5.5
# Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9  .   .   .   . 
 C:   .   .   .   .  aP9
 D:   .  aP9  .  aF9 aV9
 E:   .  dF8 aV2  .  aA9

-> Defender: attack from E1 to E2
# combat damage: to source = 1, to target = 1
# Compute time: 1.3 sec
# Average depth: 8.0
# Heuristic score: -1008

# 16/150 moves played
# Max search depth: 8
# Max search time: 5.0 sec
# Total evals at each depth: {6: 4151, 7: 2561859, 8: 5377332}
# Average eval depth: 7.7
# Average eval time: 1.3
# Average branching factor: 5.6
# Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9 dF9  .   . 
 B:  dT9  .   .   .   . 
 C:   .   .   .   .  aP9
 D:   .  aP9  .  aF9 aV9
 E:   .  dF7 aV1  .  aA9

-> Attacker: move from E2 to E3
-> Defender: move from A2 to A3

# Compute time: 1.0 sec
# Average depth: 8.0
# Heuristic score: -1035

# 18/150 moves played
# Max search depth: 8
# Max search time: 5.0 sec
# Total evals at each depth: {6: 4151, 7: 2561859, 8: 6279531}
# Average eval depth: 7.7
# Average eval time: 1.3
# Average branching factor: 5.4
# Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .  dF9  . 
 B:  dT9  .   .   .   . 
 C:   .   .   .   .  aP9
 D:   .  aP9  .  aF9 aV9
 E:   .  dF7  .  aV1 aA9

-> Attacker: repair from E4 to E3
# repaired 1 health points
# Compute time: 0.5 sec
# Average depth: 8.0
# Heuristic score: 100656

# 19/150 moves played
# Max search depth: 8
# Max search time: 5.0 sec
# Total evals at each depth: {6: 4151, 7: 2561859, 8: 6340845}
# Average eval depth: 7.7
# Average eval time: 1.3
# Average branching factor: 5.4
# Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  dF9  . 
 B:  dT9  .   .   .   . 
 C:   .   .   .   .  aP9
 D:   .  aP9  .  aF9 aV9
 E:   .  dF7  .  aV2 aA9

-> Defender: move from A3 to A4
# Compute time: 1.2 sec
# Average depth: 8.0
# Heuristic score: -1062

# 20/150 moves played
# Max search depth: 8
# Max search time: 5.0 sec
# Total evals at each depth: {6: 4151, 7: 2561859, 8: 7084269}
# Average eval depth: 7.7
# Average eval time: 1.3
# Average branching factor: 5.4
# Next player: Attacker

      0   1   2   3   4 
 A:  dA9 dT9  .   .  dF9
 B:  dT9  .   .   .   . 
 C:   .   .   .   .  aP9
 D:   .  aP9  .  aF9 aV9
 E:   .  dF7  .  aV2 aA9

-> Attacker: repair from E4 to E3
# repaired 1 health points
-> Defender: move from B0 to B1
-> Attacker: repair from E4 to E3
# repaired 1 health points

# Compute time: 0.6 sec
# Average depth: 8.0
# Heuristic score: 105652

# 23/150 moves played
# Max search depth: 8
# Max search time: 5.0 sec
# Total evals at each depth: {6: 4151, 7: 2561859, 8: 7721778}
# Average eval depth: 7.8
# Average eval time: 1.2
# Average branching factor: 5.4
# Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .   .  dF9
 B:   .  dT9  .   .   . 
 C:   .   .   .   .  aP9
 D:   .  aP9  .  aF9 aV9
 E:   .  dF7  .  aV4 aA9

-> Defender: attack from E1 to D1
# combat damage: to source = 1, to target = 1
-> Attacker: repair from E4 to E3
# repaired 1 health points
-> Defender: move from B1 to B2
-> Attacker: repair from E4 to E3
# repaired 1 health points
-> Defender: move from A4 to B4
-> Attacker: repair from E4 to E3
# repaired 1 health points
-> Defender: move from B2 to B3
-> Attacker: repair from E4 to E3
# repaired 1 health points
-> Defender: move from B3 to A3
-> Attacker: repair from E4 to E3
# repaired 1 health points

# 33/150 moves played
# Max search depth: 8
# Max search time: 5.0 sec
# Total evals at each depth: {6: 4151, 7: 2561859, 8: 11851061}
# Average eval depth: 7.8
# Average eval time: 1.2
# Average branching factor: 5.2
# Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .  dT9  . 
 B:   .   .   .   .  dF9
 C:   .   .   .   .  aP9
 D:   .  aP8  .  aF9 aV9
 E:   .  dF6  .  aV9 aA9

-> Defender: attack from B4 to C4
# combat damage: to source = 1, to target = 1
-> Attacker: move from E3 to E2
-> Defender: move from A3 to A4
-> Attacker: move from E4 to E3
-> Defender: attack from B4 to C4
# combat damage: to source = 1, to target = 1
-> Attacker: attack from E2 to E1
# combat damage: to source = 1, to target = 1
-> Defender: repair from A4 to B4
# repaired 3 health points
-> Attacker: repair from E3 to E2
# repaired 1 health points

# 41/150 moves played
# Max search depth: 8
# Max search time: 5.0 sec
# Total evals at each depth: {6: 4151, 7: 2561882, 8: 15120904}
# Average eval depth: 7.9
# Average eval time: 1.3
# Average branching factor: 4.9
# Next player: Defender

      0   1   2   3   4 
 A:  dA9 dT9  .   .  dT9
 B:   .   .   .   .  dF9
 C:   .   .   .   .  aP7
 D:   .  aP8  .  aF9 aV9
 E:   .  dF5 aV9 aA9  . 

-> Defender: attack from B4 to C4
# combat damage: to source = 1, to target = 1
-> Attacker: attack from E2 to E1
# combat damage: to source = 1, to target = 1
-> Defender: repair from A4 to B4
# repaired 3 health points
-> Attacker: attack from D1 to E1
# combat damage: to source = 1, to target = 1
-> Defender: attack from B4 to C4
# combat damage: to source = 1, to target = 1
-> Attacker: repair from E3 to E2
# repaired 1 health points
-> Defender: move from A1 to A2
-> Attacker: attack from E2 to E1
# combat damage: to source = 1, to target = 1
-> Defender: attack from B4 to C4
# combat damage: to source = 1, to target = 1
-> Attacker: attack from E2 to E1
# combat damage: to source = 1, to target = 1

# 51/150 moves played
# Max search depth: 8
# Max search time: 5.0 sec
# Total evals at each depth: {6: 4154, 7: 2561941, 8: 20078600}
# Average eval depth: 7.9
# Average eval time: 1.4
# Average branching factor: 4.8
# Next player: Defender

      0   1   2   3   4 
 A:  dA9  .  dT9  .  dT9
 B:   .   .   .   .  dF7
 C:   .   .   .   .  aP4
 D:   .  aP7  .  aF9 aV9
 E:   .  dF1 aV7 aA9  . 

-> Defender: attack from B4 to C4
# combat damage: to source = 1, to target = 1

      0   1   2   3   4 
 A:  dA9  .  dT9  .  dT9
 B:   .   .   .   .  dF6
 C:   .   .   .   .  aP3
 D:   .  aP7  .  aF9 aV9
 E:   .  dF1 aV7 aA9  . 

-> Attacker: attack from E2 to E1
# combat damage: to source = 1, to target = 1

      0   1   2   3   4 
 A:  dA9  .  dT9  .  dT9
 B:   .   .   .   .  dF6
 C:   .   .   .   .  aP3
 D:   .  aP7  .  aF9 aV9
 E:   .   .  aV6 aA9  . 

-> Defender: repair from A4 to B4
# repaired 3 health points

      0   1   2   3   4 
 A:  dA9  .  dT9  .  dT9
 B:   .   .   .   .  dF9
 C:   .   .   .   .  aP3
 D:   .  aP7  .  aF9 aV9
 E:   .   .  aV6 aA9  . 

-> Attacker: move from D1 to C1
-> Defender: attack from B4 to C4
# combat damage: to source = 1, to target = 1
-> Attacker: move from C1 to B1
# Compute time: 1.9 sec
-> Defender: move from A0 to A1
-> Attacker: move from D3 to C3
-> Defender: attack from A1 to B1
# combat damage: to source = 3, to target = 3

      0   1   2   3   4 
 A:   .  dA6 dT9  .  dT9
 B:   .  aP4  .   .  dF8
 C:   .   .   .  aF9 aP2
 D:   .   .   .   .  aV9
 E:   .   .  aV6 aA9  . 

-> Attacker: move from D4 to D3

# Next player: Defender

      0   1   2   3   4 
 A:   .  dA6 dT9  .  dT9
 B:   .  aP4  .   .  dF8
 C:   .   .   .  aF9 aP2
 D:   .   .   .  aV9  . 
 E:   .   .  aV6 aA9  . 

-> Defender: repair from A4 to B4
# repaired 3 health points

      0   1   2   3   4 
 A:   .  dA6 dT9  .  dT9
 B:   .  aP4  .   .  dF9
 C:   .   .   .  aF9 aP2
 D:   .   .   .  aV9  . 
 E:   .   .  aV6 aA9  . 

-> Attacker: repair from E3 to E2
# repaired 1 health points

      0   1   2   3   4 
 A:   .  dA6 dT9  .  dT9
 B:   .  aP4  .   .  dF9
 C:   .   .   .  aF9 aP2
 D:   .   .   .  aV9  . 
 E:   .   .  aV7 aA9  . 

-> Defender: repair from A2 to A1
# repaired 3 health points

      0   1   2   3   4 
 A:   .  dA9 dT9  .  dT9
 B:   .  aP4  .   .  dF9
 C:   .   .   .  aF9 aP2
 D:   .   .   .  aV9  . 
 E:   .   .  aV7 aA9  . 

-> Attacker: move from C3 to C2
-> Defender: attack from B4 to C4
# combat damage: to source = 1, to target = 1
-> Attacker: repair from E3 to E2
# repaired 1 health points
-> Defender: attack from B4 to C4
# combat damage: to source = 1, to target = 1
-> Attacker: move from D3 to C3
-> Defender: repair from A4 to B4
# repaired 3 health points
-> Attacker: move from C3 to B3
-> Defender: attack from B4 to B3
# combat damage: to source = 1, to target = 1
-> Attacker: move from B3 to B2
-> Defender: repair from A4 to B4
# repaired 3 health points

      0   1   2   3   4 
 A:   .  dA9 dT9  .  dT9
 B:   .  aP4 aV8  .  dF9
 C:   .   .  aF9  .   . 
 D:   .   .   .   .   . 
 E:   .   .  aV8 aA9  . 

-> Attacker: repair from E3 to E2
# repaired 1 health points

      0   1   2   3   4 
 A:   .  dA9 dT9  .  dT9
 B:   .  aP4 aV8  .  dF9
 C:   .   .  aF9  .   . 
 D:   .   .   .   .   . 
 E:   .   .  aV9 aA9  . 

-> Defender: move from B4 to C4
-> Attacker: move from E2 to E1
-> Defender: move from A4 to A3
-> Attacker: move from E3 to D3
-> Defender: move from C4 to D4

# Next player: Attacker

      0   1   2   3   4 
 A:   .  dA9 dT9 dT9  . 
 B:   .  aP4 aV8  .   . 
 C:   .   .  aF9  .   . 
 D:   .   .   .  aA9 dF9
 E:   .  aV9  .   .   . 

-> Attacker: attack from B1 to A1
# combat damage: to source = 3, to target = 3

# 81/150 moves played
# Max search depth: 8
# Max search time: 5.0 sec
# Total evals at each depth: {2: 3, 8: 24985977, 9: 12876662, 6: 7251, 4: 750, 3: 27, 5: 635, 7: 2577047}
# Average eval depth: 8.3
# Average eval time: 1.6
# Average branching factor: 4.5
# Next player: Defender

      0   1   2   3   4 
 A:   .  dA6 dT9 dT9  . 
 B:   .  aP1 aV8  .   . 
 C:   .   .  aF9  .   . 
 D:   .   .   .  aA9 dF9
 E:   .  aV9  .   .   . 

-> Defender: attack from A2 to B2
# combat damage: to source = 6, to target = 6

# 82/150 moves played
# Next player: Attacker

      0   1   2   3   4 
 A:   .  dA6 dT3 dT9  . 
 B:   .  aP1 aV2  .   . 
 C:   .   .  aF9  .   . 
 D:   .   .   .  aA9 dF9
 E:   .  aV9  .   .   . 

-> Attacker: move from E1 to D1
-> Defender: repair from A2 to A1
# repaired 3 health points
-> Attacker: move from D1 to C1

      0   1   2   3   4 
 A:   .  dA9 dT3 dT9  . 
 B:   .  aP1 aV2  .   . 
 C:   .  aV9 aF9  .   . 
 D:   .   .   .  aA9 dF9
 E:   .   .   .   .   . 

-> Defender: attack from A2 to B2
# combat damage: to source = 6, to target = 6

      0   1   2   3   4 
 A:   .  dA9  .  dT9  . 
 B:   .  aP1  .   .   . 
 C:   .  aV9 aF9  .   . 
 D:   .   .   .  aA9 dF9
 E:   .   .   .   .   . 

-> Attacker: attack from B1 to A1
# combat damage: to source = 3, to target = 3

      0   1   2   3   4 
 A:   .  dA6  .  dT9  . 
 B:   .   .   .   .   . 
 C:   .  aV9 aF9  .   . 
 D:   .   .   .  aA9 dF9
 E:   .   .   .   .   . 

-> Defender: move from A1 to A2
-> Attacker: move from C1 to B1
-> Defender: attack from D4 to D3
# combat damage: to source = 1, to target = 1
-> Attacker: move from B1 to A1

      0   1   2   3   4 
 A:   .  aV9 dA6 dT9  . 
 B:   .   .   .   .   . 
 C:   .   .  aF9  .   . 
 D:   .   .   .  aA8 dF8
 E:   .   .   .   .   . 

-> Defender: repair from A3 to A2
# repaired 3 health points

      0   1   2   3   4 
 A:   .  aV9 dA9 dT9  . 
 B:   .   .   .   .   . 
 C:   .   .  aF9  .   . 
 D:   .   .   .  aA8 dF8
 E:   .   .   .   .   . 

-> Attacker: attack from A1 to A2
# combat damage: to source = 3, to target = 9
# Heuristic score: 2147483554

# 93/150 moves played
# Max search depth: 6
# Max search time: 5.0 sec
# Total evals at each depth: {2: 13, 8: 26069529, 9: 13120997, 6: 10077, 4: 919, 3: 64, 5: 1227, 1: 2, 7: 2592873, 10: 483806}
# Average eval depth: 8.3
# Average eval time: 1.4
# Average branching factor: 4.4
# Next player: Defender

      0   1   2   3   4 
 A:   .  aV6  .  dT9  . 
 B:   .   .   .   .   . 
 C:   .   .  aF9  .   . 
 D:   .   .   .  aA8 dF8
 E:   .   .   .   .   . 

Attacker wins in 93 moves!
```