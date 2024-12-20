---
title: Combat Rules
weight: 90
prev: /rules/adventuring_rules
next: /rules/magic_rules
---

## Combat procedure

During combat, time is tracked using rounds.
The factions involved in the fight alternate taking turns with a character or passing.
Each character can only take a turn once per round.
When all factions pass consecutively, the round ends.
At the end of the round, you must make morale checks if necessary (see [Morale]({{< relref "#morale" >}})).

The faction holding the initiative can decide which faction acts first at the start of each round.
Initiative is held by the faction which started the fight, or is assigned randomly if that's unclear.

{{% details title="Example" closed="true" %}}

Balthasar, Sybilla, and Theobald are fighting against three bandits and their leader.
The bandits started the fight and have the initiative.
The round plays out in this sequence:

* The GM decides to take turn with the bandit leader first.
* The Players decide to take turn with Sybilla.
* The GM takes turn with a bandit.
* The Players decide to pass, rather than activating.
* The GM takes turn with another bandit. Since he didn't pass, play goes back to the players.
* The Players take turn with Balthasar.
* The GM takes turn with the last bandit.
* The players take turn with Theobald.
* The GM must pass because all bandits have taken their turn.
* The Players must pass as well. Since all factions have passed consecutively, the round ends.

{{% /details %}}


## Taking a turn

During their turn, characters can either move their full speed (typically 2 zones, or 4 zones if sprinting), or move half their speed (1 zone, or 2 zones if sprinting) and perform a _bonus action_ and a _main action_ (or a second _bonus action_).
It is possible to split movement before, after, and between actions.

Alternatively, characters can take their turn outside the normal round sequence to perform a _reaction_ in response to a specific trigger.
A reaction takes the whole turn and doesn't allow for movement or a bonus action.

**Main actions**: attacking, invoking a profane power, distracting, intimidating, taunting, using a consumable item (a _medicine box_, an _antidote_, etc.), or any other action taking a round.

**Bonus actions**: picking up or putting down something, equipping or unequipping an item, opening a door, pulling a lever, saying a few words, or any other action which only takes a second or two.

**Reactions**: countering, dodging, guarding, hiding behind someone else from an attack, or performing an opportunity attack.


## Attacking

Attacking an enemy is a main action.
Melee attacks can only target nearby enemies, ranged attacks can target visible enemies within range.

Unarmed attacks inflict d4 impaired damage.
Armed attacks inflict damage depending on the weapon used.
The attacker may choose to voluntarily hold back an attack to inflict impaired damage.
If the target is nearby and _incapacitated_, the attacker may decide to deliver a death blow and kill them.

{{% details title="Example" closed="true" %}}

Balthasar attacks a nearby bandit with a sword (d6 damage).
The bandit directly takes d6 damage.
He rolls a 4, reducing his health from 8 to 4 and incapacitating him.

Sybilla later attacks the same bandit with a dagger.
Since he is nearby and _incapacitated_, she slits his throats and kills him on the spot.

{{% /details %}}

Typically, attacks automatically hit, however passing a WIT save is required to hit in challenging circumstances.
For melee attacks: when the target isn't visible, for example when fighting in complete darkness.
For ranged attacks: when the target is beyond half range, or the attack is made while moving.
It's impossible to target a character beyond half range while moving, or a character who is not visible, with a ranged attack.

{{% details title="Example" closed="true" %}}

A bandit shoots Balthasar with a bow (d6 damage, range 8).
They are at 5 zones of distance from each other, so the bandit must pass a WIT save to hit.
He rolls 5 on a d20, a pass, therefore Balthasar takes d6 damage.

Theobald attacks a nearby bandit with his sword (d6 damage) in complete darkness.
Since he can't see, he must pass a WIT save to hit.
He rolls a 20 on the d20, a miss, so the bandit takes no damage.

{{% /details %}}


## Reacting to an attack

Targets of an attack may attempt to dodge, counter, or hide behind a nearby character as a reaction.
The choice must be made before rolling for damage but after the WIT save to hit, if required.
Targets who are unaware of their attacker can't react.
Some skills grant the ability to react multiple times in a round, however it is never possible to react more than once against a single attack.

**Dodge**.
Make an AGI save: on a pass the attack misses.

**Counter**.
Perform a suitable attack against the attacker.
Both characters roll for damage at the same time.
Whoever suffers the most damage, after reductions, is hit first.
In case of a tie both characters are hit at the same time.
Attacks directed at multiple targets (such as blast attacks) can only be countered by a single target.

**Hide behind a nearby character**.
Choose a nearby character who becomes the new target of the attack, swapping position with the original target.
They can perform their own reaction if they haven't taken their turn yet.
It is only possible to hide behind a willing ally or a currently grabbed enemy.

{{% details title="Example" closed="true" %}}

Sybilla attacks a bandit (AGI 8) with her musket (d8 damage).
The bandit hasn't acted yet during this turn and decides to dodge.
He rolls a 2 on a d20, lower than their AGI, so the attack misses.

Theobald attacks the bandit leader with his spear (d6 damage).
The bandit leader decides to counter with his battleaxe (d8 damage).
They both roll for damage: Theobald inflicts 4 damage, the bandit leader inflicts 5 damage.
Theobald however is holding a _shield_ and wearing _light armour_, so he has an armour value of 2 and would only suffer 3 damage.
Because of this, the bandit leader is hit for 4 damage first.
This is enough to inflict critical damage and incapacitate him, so Theobald suffers no damage at all!

{{% /details %}}


## Guarding

A character who is nearby the target of an attack can react by guarding them.
They become the new target of the attack.
Since they have spent their reaction to guard, they can't normally counter or dodge.


## Opportunity attacks

Characters can make an opportunity attack as a reaction against nearby enemies moving away.
The target's movement is interrupted and the attack is resolved, after which their turn resumes if they are still alive and  conscious.


## Cover

**Cover** offers physical protection.
The target's armour value is increased by 1 (as usual never above 3), unless the attack is powerful enough to penetrate the cover.

**Barriers** completely block attacks.
If a character peaks behind cover to attack, they can be still countered and the barrier only counts as cover.


## Blast attacks

Blast attacks hit a zone and target all characters in it and on its borders.

Melee blast attacks hit the attacker's zone and don't target the attacker themselves.
They can't miss.

Ranged blast attacks hit any zone within range and target the attacker as well if directed at their own zone.
If they miss, they are deviated and hit a random neighbouring zone.
Assign a number to each neighbouring zone and roll a die to determine which one is hit.


## Attack stunts

The attacker can propose an alternative effect to the target instead of inflicting damage (cutting a limb, forcing a surrender, etc.).
This must be done before the target decides whether to react, and the alternative effect must make sense and be approved by the GM.
The target can either accept the proposed effect or resolve the attack as normal.
It isn't possible to react if the effect is accepted.

{{% details title="Example" closed="true" %}}

Balthasar attacks a bandit with 3 health left with his sword (d6 damage).
He doesn't want to kill him, so he proposes an attack stunt: instead of taking damage, the bandit is disarmed and _incapacitated_ until the end of the stretch.
Given his low health, the bandit happily accepts the alternative effect.

{{% /details %}}


## Non-lethal attacks

Non-lethal attacks don't inflict damage, but instead force the target to pass a save or suffer negative consequences.
They can be dodged and countered and can be used to counter.
If a normal attack is countered by a non-lethal attack, or viceversa, the non-lethal attack is resolved last.
If a non-lethal attack is countered by a non-lethal attack, the counter is resolved last.

**Disarm**.
Target a nearby character.
The target must pass a STR save or drop a weapon chosen by the attacker.
If the attacker has a free hand, they can grab the weapon.

**Shove**.
Target a nearby character.
The target must pass a STR save or the attacker can force them to move by half a zone, and eventually push them into a hazard (pits, traps, lava, etc.).

**Grapple**.
Target a nearby character.
The target must pass a STR save or the attacker grabs them.
Grabbed characters can't act, and can only spend their turn to attempt to break free by passing a STR save.
Allies can also spend a main action to attempt to free them with a STR save.
The grabbing character can keep hold while moving together with the grabbed character, attempting to disarm them, hiding behind them as reaction to an attack, or attacking with a one-handed weapon or an unarmed attack.
Any other action frees the grabbed character.

{{% details title="Example" closed="true" %}}

Theobald attempts to grab a bandit.
The bandit reacts by countering with his axe, inflicting d6 damage: he rolls a 1, just a scratch!
Since Theobald is still standing, the bandit must now attempt a STR save to avoid being grabbed, but fails.

On his next turn, Theobald attacks the bandit with a dagger, inflicting d4 damage.
The bandit can't react since he is grabbed, but he is still alive and conscious despite suffering damage.
On his turn, he can only attempt to break free by passing a STR save.
He succeeds, however his turn has already been consumed by the attempt to break free and he can't do anything else.

{{% /details %}}


## Sneaking in combat

Characters who launch an ambush are concealed at the start of the fight.
Hiding later during the fight requires being out of sight for a while or a major distraction.
The GM should keep the position of concealed characters hidden from the Players, and should control their characters so that they realistically ignore concealed Player characters.

At the start of the fight, concealed characters play a bonus round, during which only they can act.
Enemies attacked by a concealed character are taken by surprise and can't react.

Concealed characters are revealed when they make noise or become visible to the enemy.
Actions which might reveal a character include: attacking, invoking a power, talking, moving without sneaking, walking right in front of the enemy, etc.


## Morale

Morale checks are made at the end of the round.
Groups must make a morale check if they were reduced to half or less their original numbers in the past round.
Characters fighting alone must make a morale check if they were reduced to half or less their maximum health in the past round.

Make a group WIT save: those who fail must surrender or retreat, but those who pass aren't subject to morale for the remainder of the stretch.
Characters who are immune to fear are also immune to morale.


## Surrendering

Characters can spend their turn to surrender, throwing their weapons away, putting their hands up, etc.
Their turn is wasted, but they might be spared by the enemy.
Surrendering characters who are harmed by the enemy can resume fighting normally, even if they previously failed a morale check.


## Chases

Short chases can be played out using the combat rules.
Chases over long distances can be resolved with the fleeing character making a group AGI save: those who pass escape, those who fail are reached by the pursuers.
If the fleeing characters can move faster or keep the speed for longer, the save is not required.
If the opposite is true, escaping is impossible.


## Combat gear degradation

After a fight, make a durability roll for all weapons and armour which were used at least once.
This represents damage to the equipment and depleting ammunition.

_Damaged_ weapons and shields are destroyed if used to attack and a 1 is rolled on the damage die.
_Damaged_ shields and armour are destroyed by attacks inflicting at least 8 damage before armour reduction (shields are destroyed first).

Many mundane items can be used as improvised weapons.
They work as a _simple hand weapon_ or a _simple great weapon_, but are handled as if they were already _damaged_.


## Effects of size in combat

**Attack damage**.
Smaller characters halve damage for each size of difference.
Larger characters double che number of dice they roll for damage for each size of difference, and can distribute dice from melee attacks among several smaller targets as they see fit.
Damage from blast attacks isn't modified.

**Range**.
Massive characters can make melee attacks up to range 1.
When targeting a character of different size with a ranged attack, the maximum range is halved for each size of difference if the target is smaller, or doubled for each size of difference if the target is larger.
This rule doesn't apply to _blast_ attacks.

**Non-lethal attacks**.
Smaller characters can't resist with a STR save when a larger character disarms, shoves, or grabs them (they can however still attempt to dodge or counter), and can't free themselves from a grab with a STR save.
Larger characters can't be disarmed, shoved, or grabbed by smaller characters.

**Shoving and grabbing**.
Larger characters shoving a smaller one inflict d4 direct damage and double the range of the shove for each category of difference (1, 2, 4, or 8 zones if the target is respectively 1, 2, 3, or 4 sizes smaller).
Larger characters need only one hand (or any other appendage) to grab a smaller target, and besides having that hand occupied they can perform any action while keeping hold.

{{% details title="Example" closed="true" %}}

A wyrm is a large monster with two natural attacks: _bite, talons, and tail_ (d8 damage) and _fire breath_ (d6 damage, _blast_).

Its bite attack inflicts 2d8 damage against a single medium-sized character, and 4d8 damage against a single small character.
Alternatively, it could be targeted at multiple smaller characters, for example: two medium-sized targets (d8 damage each), three small-sized characters (two suffering d8 damage, one suffering 2d8 damage), a medium-sized and a small target (the former taking d8 damage, the latter 2d8 damage), etc.

Its fire breath attack has the _blast_ property, therefore it always inflicts d6 damage, no matter the target's size.

A human attacking a wyrm with a sword (d6 damage) would halve damage: for example, a roll of 3 would only inflict 2 damage.
A human attacking with a bow (d6 damage, range 8) would halve damage, but would also double the range to 16.
A human throwing a _fire bomb_ (d6 damage, range 2, blast) would inflict full damage against the wyrm, thanks to the _blast_ property.

{{% /details %}}


## Mounts and vehicles in combat

Mounts and riders, as well as vehicles and passengers, take turn together.
Individual mounts, riders, and passengers can choose not to do anything and reserve their turn to react.
If the mount or vehicle moves at full speed, riders and passengers can't take any action nor react.

Mounts, vehicles, riders, and passengers are targeted individually by enemy attacks.
Attacks made by riders and passengers are impaired.

Mounting or dismounting, or getting on or on off a vehicle, counts as moving half a zone.
If the mount or vehicle is moving, an AGI save is required to avoid falling, suffering d4 direct damage.
