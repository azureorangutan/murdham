---
title: Core Rules
weight: 20
prev: /rules/introduction
next: /rules/player_characters
---

## Tracking time

Time is an important resource in the game, and is tracked using three units.

**Rounds**.
About 6 seconds.
Rounds are used during fights, pursuits, and other situations where a split second can make the difference.

**Stretches**.
About 10 minutes (100 rounds).
Stretches are used when exploring adventure sites, resolving encounters, etc.

**Watches**.
About 8 hours (48 stretches).
Watches are used when travelling, resting, etc.
A day is made of three watches: morning (from sunrise), afternoon (from noon), and night (from sunset).


## Abilities

The three abilities represent a character's basic proficiencies.
Each one has a score ranging between 1 and 19.

**Strength (STR)**.
Fortitude, might, physical power, toughness.

**Agility (AGI)**.
Athleticism, dexterity, reflexes, swiftness.

**Wits (WIT)**.
Awareness, charisma, cunning, willpower.


## Saves

Characters must make a save when threatened or when they attempt to do something challenging or risky, where the outcome is uncertain and failure has consequences.
They pass if they roll equal or less than the most relevant ability score on a d20, otherwise they fail and suffer some sort of penalty.

{{% details title="Example" closed="true" %}}

Balthasar (AGI 7), Sybilla (AGI 10), and Theobald (AGI 6) want to climb a rocky cliff.
The GM adjudicates that this is a risky and challenging activity requiring an AGI save: whoever fails will fall and suffer d4 damage.
Theobald thinks it's too risky for him, and declares he would rather not attempt the climb.
Balthasar rolls a 6 on a d20: he passes and reaches the top of the climb unharmed.
Sybilla rolls a 13 on a d20: she isn't so lucky and falls, suffering damage and going back to where she started.

{{% /details %}}

A save should only be required for challenging, risky actions.
Trivial actions should succeed automatically.
Impossible or almost impossible actions should fail automatically.

{{% details title="Example" closed="true" %}}

Climbing a ladder is trivial: it can be done without rolling for a save.
Climbing a vertical rocky surface, or a slippery ladder, is challenging: a save is required.
Climbing a sheer, smooth surface is impossible: characters just can't do it unless they use a tool (rope, a ladder, etc.).

{{% /details %}}

Sometimes, multiple characters must save against the same threat at once.
In this situation, the GM can ask to make a group save to speed up the game.
Roll the d20 only once and compare the result with each individual's ability score to see who passes and who fails.

{{% details title="Example" closed="true" %}}

Balthasar (WIT 12), Sybilla (WIT 6), and Theobald (WIT 9) encounter a terrifying creature.
The GM asks them to pass a group WIT save to avoid being overwhelmed by fear.
One of the Players rolls a d20 and gets a 9: Balthasar and Theobald pass, but Sybilla fails.

{{% /details %}}


## Contests

Characters who are directly competing against each other must make a contest to determine the winner.
All competing characters roll a d20 and subtract the result from the most relevant ability score.
They are then ranked from highest to lowest total.
Ties are broken by repeating the contest among the tying characters.

{{% details title="Example" closed="true" %}}

Balthasar (STR 5), Sybilla (STR 8), and Theobald (STR 9) decide to engage in a friendly rock-throwing contest.
The GM decides that the winner will be decided with a STR contest.
Balthasar rolls a 6 on a d20, for a total of 5−6 = −1.
Sybilla rolls a 5, for a total of 8−5 = 3.
Theobald rolls a 20, for a total of 9−20 = −11.
The ranking is as follows: Sybilla first, then Balthasar, then Theobald.

{{% /details %}}

Roll for a contest only if it's uncertain who would win.
Characters who are clearly superior win automatically without rolling.

{{% details title="Example" closed="true" %}}

Two people racing against each other would make an AGI contest to see who wins.
A person racing against a horse in an open filed would always lose, no roll required.

{{% /details %}}

Prefer a save over a contest whenever possible, especially in asymmetric situations where a side is clearly more at risk.
Use contests only for competitions and other symmetric situations.

{{% details title="Example" closed="true" %}}

Two characters engaging in arm wrestling should determine the winner with a STR contest: it is a symmetric competition.
A character who is pushed by someone else should make a STR save to stand their ground: it is an asymmetric challenge, and the pusher is not at risk.

{{% /details %}}


## Consequences of failure

Failing a save or losing a contest should always have negative consequences, but doesn't necessarily mean a failure in the action itself.
In particular, failure should never halt progress through the scenario, so in such a situation the GM might want to let the action succeed but at a cost.
The following are typical consequences for failure:

* The character suffers harm (damage, corruption, a negative condition, etc.).

* Time is lost, resources are exhausted, equipment is damaged, etc.

* The character angers someone, attracts unwanted attention, is put on a spot, etc.

{{% details title="Example" closed="true" %}}

The most intuitive penalty for failing a save while climbing would be falling and taking damage.
However, depending on the situation, the GM might just rule that it takes longer to make the climb, or that an item falls from the character's backpack and breaks.

{{% /details %}}


## Skills, traits, and conditions

Skills, traits, and conditions are keywords describing a character's properties.
There are no degrees or ranks: a character either has a keyword or doesn't.
Skills are acquired by training and learning, traits are other permanent properties, and conditions are transitory properties.

The description of each keyword lists the most common ways in which it can influence the game, but don't feel limited by this: you should take them into account any time it makes sense within the game narrative.


## Skill proficiency

If a skill grants proficiency at certain tasks, the character reduces the difficulty of relevant actions:

* Challenging actions, which would normally require a save, are trivial and succeed automatically.

* Almost impossible actions, which normally couldn't be attempted, can be attempted with a save.
The skill description might provide a few examples, but don't feel limited to those.

{{% details title="Example" closed="true" %}}

Sybilla (AGI 10) and Beatrice (AGI 8) want to climb a rocky cliff, which requires passing an AGI save.
Beatrice, however, has the _climb_ skill, which grants proficiency at climbing.
With this skill, she can succeed automatically, without needing to roll.
Sybilla doesn't have the skill, so she must roll.

Later they find themselves in a similar situation, but a torrential rain starts pouring, making the surface slippery.
The GM rules that it's practically impossible for most people to climb safely, so Sybilla can't even attempt to do it.
Since Beatrice has the _climb_ skill, however, the GM allows her to attempt to do it with an AGI save.
Once she is on top, she rolls out a _rope ladder_ she had in her backpack to let Beatrice up.
The GM rules that climbing the ladder is trivial, even under the rain, so Beatrice can just climb up without risks.

{{% /details %}}


## Health and damage

Characters have maximum health equal to their STR.
When they suffer damage, they reduce health by an equal amount, at most down to 0.

**Critical damage**.
Characters who lose half or more of their remaining health at once suffer critical damage: they are _incapacitated_ until the end of the stretch.
_Incapacitated_ characters are unconscious or in terrible pain and are completely unable to act and defenceless.

**Mortal wound**.
Characters who lose all their remaining health suffer critical damage and additionally are _dying_.
_Dying_ characters die at the end of the next round unless they recover at least 1 health before then.
Even if they recover, they suffer an injury determined by rolling on the table below.

**Instant death**.
Characters who suffer damage matching or exceeding their maximum health at once are killed on the spot.

{{% details title="Example" closed="true" %}}

Theobald has STR 9, therefore his maximum health is 9 as well.
When at full health, he suffers critical damage if he takes 5 or more damage at once.
If he takes damage and loses health, the amount necessary to trigger critical damage is reduced accordingly.
For example, once he is reduced to 4 health, he would suffer critical damage with just 2 points of damage.
No matter what his current health is, he dies immediately if he suffers 9 or more damage at once.

{{% /details %}}

|  D12  | Injury                                                                                                                                                                                                                      |
| :---: | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|   1   | **Scarred**. The wound leaves a permanent mark, but there are otherwise no negative consequences.  Describe the scar as vividly as you can!                                                                                 |
|   2   | **Fear**. Suffer from the _frightened_ condition, relating to whom or what caused your injury, until a full rest.                                                                                                           |
|   3   | **Painful wound**. STR is temporarily reduced by d4.                                                                                                                                                                        |
|   4   | **Cracked bones**. AGI is temporarily reduced by d4.                                                                                                                                                                        |
|   5   | **Concussion**. WIT is temporarily reduced by d4.                                                                                                                                                                           |
|   6   | **Injured leg**. Suffer from the _hobbled_ condition until a full rest.                                                                                                                                                     |
|   7   | **Injured arm**. Suffer from the _arm injury_ condition until a full rest. Roll a d2 to determine which arm is affected: 1) primary arm, 2) secondary arm. The other arm is impacted if this injury is taken a second time. |
|   8   | **Blinded eye**. Suffer from the _eye injury_ condition until a full rest. If the other eye is blinded, suffer from the _blind_ condition instead.                                                                          |
|   9   | **Deafened**. Suffer from the _deaf_ condition until a full rest.                                                                                                                                                           |
|  10   | **Smashed mouth**. Suffer from the _silenced_ condition until a full rest.                                                                                                                                                  |
|  11   | **Infected wound**. Acquire the _sick_ condition, with a disease chosen by the GM. You can't resist with a STR save.                                                                                                        |
|  12   | **Bloody mess**. Acquire the _bleeding_ condition.                                                                                                                                                                          |


## Damage rolls

Damage is usually rolled on a d4, d6, d8, d10, or d12.

**Exploding damage**.
If you roll the maximum possible number, you must re-roll the same die and add the new result to the previous roll minus 1.
Keep doing this as long as you keep rolling the maximum.

**Armour value**.
Reduces incoming damage by a matching amount, but not damage denoted as "direct".
Increased by armour and shields, but can never exceed 3.

**Impaired and enhanced damage**.
Roll twice and keep the lower or higher result respectively.
Factors enhancing and impairing damage at the same time cancel each other out on a one-to-one basis.
Damage which is impaired multiple times is completely ineffective.
Multiple enhancements have no effect.

{{% details title="Example" closed="true" %}}

Balthasar takes d6 damage and rolls a 6, the maximum: he must roll the d6 again and add 5 (6−1).
He rolls another 6: he must roll the d6 a third time and add 10 (5+5) to it.
Finally, he rolls 3, for a grand total of 13 damage: quite impressive!

Sybilla suffers d6 enhanced damage, therefore she must roll a d6 twice and keep the higher result.
She rolls a 4 and a 6.
She must re-roll the second die and add 5: she rolls a 3 for a total of 5+3 = 8 damage.
The results of the two rolls are 4 and 8: since 8 is the greater roll, she suffers 8 damage.

Balthasar suffers d8 impaired damage, therefore he must roll a d8 twice and keep the lower result.
He rolls 3 and 7, resulting in only 3 damage.

{{% /details %}}


## Ability changes

Abilities can change permanently or temporarily due to various effects.
Characters die if STR is reduced to 0, are paralysed if AGI is reduced to 0, and are comatose if WIT is reduced to 0.
If the reduction is permanent, the character is effectively defeated.
Abilities can never exceed 19, or 15 for humans, not even temporarily.
Temporarily altered abilities are restored after a full rest.

When STR is reduced below current health, health is reduced to match.
However, the character doesn't suffer critical damage when health is reduced in this way.

{{% details title="Example" closed="true" %}}

Theobald has STR 9 and his current health is 6.
His STR is temporarily reduced to 7, and his health remains at 6 since it's still lower.
Later, his STR is further reduced down to 3.
Now, his health must also be reduced to 3.
Even though he has lost more than half his health at once, he doesn't suffer critical damage since it was caused by STR loss rather than damage.

{{% /details %}}


## Corruption

Corruption represents spiritual taint and is caused by the use of magic or the proximity of strong otherworldly powers.
Characters who suffer corruption must roll a d12.
If they roll lower or equal than their current corruption, including the amount just suffered, they suffer soulblight:
they take direct damage equal to the rolled number, and corruption is reduced by the same amount.

{{% details title="Example" closed="true" %}}

Balthasar starts with 0 corruption and suffers 2 corruption.
He rolls 5 on a d12, meaning that nothing else happens.

He later suffers 3 corruption, bringing the total to 5.
This time, he rolls a 1 on a d12, meaning he suffers soulblight.
He takes 1 point of direct damage and his corruption is reduced to 4.

{{% /details %}}

Characters who suffer critical damage due to soulblight also develop a permanent mutation, determined by rolling on the table below or chosen by the GM.

{{% details title="Example" closed="true" %}}

Balthasar has 4 remaining health and suffers 1 corruption, bringing the total from 4 to 5.
He rolls a 3 on a d12: soulblight!
His corruption is reduced to 2, but he suffers 3 damage: enough to inflict critical damage!
Not only he is _incapacitated_, but he must also roll on the Mutations table.
He rolls 9 on a d12 and 10 on a d20: the result is "Bestial Legs".
He must roll a d6 and gets a 4, meaning his legs turn into bird legs!

{{% /details %}}

|  D12  |  D20  | Mutation                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :---: | :---: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|  1-3  |  1-2  | **Personality change**. Your personality changes for the worst. D6: 1) irascible; 2) apathetic; 3) sad; 4) proud; 5) mean; 6) pessimistic.                                                                                                                                                                                                                                                                                                                                                                    |
|       |  3-4  | **Addiction**. You gain the _addicted_ trait. Roll a d6 for what you are addicted to: 1) _alcoholic drink_; 2) _darkroot_; 3) _pipe & pipe-weed_; 4) _madcap mushrooms_; 5) _crimson weed_; 6) _corpse mandrake_.                                                                                                                                                                                                                                                                                             |
|       |  5-6  | **Unnatural appetite**. You develop an unnatural appetite: you must satisfy it instead of eating normal food or drinking water. D6: 1) drink blood; 2) drink tears; 3) drink quicksilver; 4) eat mud; 5) eat human flesh; 6) eat maggots.                                                                                                                                                                                                                                                                     |
|       |  7-8  | **Disgusting habit**. You develop an annoying or disgusting habit. D6: 1) eat your hair; 2) scratch your butt; 3) fart; 4) pick your nose; 5) spit; 6) belch loudly.                                                                                                                                                                                                                                                                                                                                          |
|       | 9-10  | **Compulsion**. You develop an irresistible desire to do something: when tempted, you must pass a WIT save to resist. D6: 1) make things burn; 2) destroy beautiful things; 3) keep your stuff perfectly clean; 4) insult figures of authority; 5) buy unnecessarily expensive things; 6) steal.                                                                                                                                                                                                              |
|       | 11-12 | **Revulsion**. You must pass a WIT save to force yourself to approach or touch the source of your revulsion. D6: 1) metal; 2) animals; 3) people; 4) mushrooms; 5) water; 6) dirt.                                                                                                                                                                                                                                                                                                                            |
|       | 13-14 | **Irrational phobia**. You are afraid of something irrational. D6: 1) rodents; 2) salt water; 3) religious paraphernalia; 4) a particular colour; 5) insects; 6) wizards.                                                                                                                                                                                                                                                                                                                                     |
|       | 15-16 | **Irrational hatred**. You must pass a WIT save to keep yourself from act aggressively against the target of your hatred. D6: 1) a specific group of people; 2) wizards; 3) priests; 4) demons; 5) undead; 6) animals.                                                                                                                                                                                                                                                                                        |
|       | 17-18 | **Uncontrollable bloodlust**. When blood is spilt nearby, you must pass a WIT save or become _frenzied_ until the end of the stretch. If you pass you are immune to this effect until the end of the stretch.                                                                                                                                                                                                                                                                                                 |
|       | 19-20 | **Telepathy**. You can communicate telepathically with a person you can see within range 8. While doing this you must concentrate and can't perform any other activity requiring your focus.                                                                                                                                                                                                                                                                                                                  |
|  4-5  |  1-3  | **Atrophy**. Your STR is permanently reduced by d4.                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
|       |  4-6  | **Bulging muscles**. Your STR is permanently increased by d4. It can go above 15, but not above 19.                                                                                                                                                                                                                                                                                                                                                                                                           |
|       |  7-9  | **Painful boils**. Your AGI is permanently reduced by d4.                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
|       | 10-12 | **Flexible joints**. Your AGI is permanently increased by d4. It can go above 15, but not above 19.                                                                                                                                                                                                                                                                                                                                                                                                           |
|       | 13-15 | **Brain rot**. Your WIT is permanently reduced by d4.                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
|       | 16-18 | **Keen senses**. Your WIT is permanently increased by d4. It can go above 15, but not above 19.                                                                                                                                                                                                                                                                                                                                                                                                               |
|       | 19-20 | **Curse**. Your maximum omens are permanently reduced by 1. Reroll if they are already 0.                                                                                                                                                                                                                                                                                                                                                                                                                     |
|  6-8  |  1-4  | **Unusual skin**. Your skin develop an unusual colour, pattern, or appearance. D6: 1) bright red; 2) corpse-like flesh; 3) multi-chromatic; 4) metallic; 5) hairless; 6) covered in strange markings.                                                                                                                                                                                                                                                                                                         |
|       |  5-8  | **Unusual eyes**. Your eyes develop an unusual colour, pattern, or appearance. This doesn't affect your ability to see.  D6: 1) empty sockets; 2) eyes like a starry night; 3) burning flames; 4) eye-stalks; 5) lizard-like; 6) blood red.                                                                                                                                                                                                                                                                   |
|       | 9-12  | **Unusual hair**. Your hair develops an unusual colour, pattern, or appearance. D6: 1) bright red and waving like a living flame; 2) crawling maggots; 3) silvery white and strong like metal; 4) moving in spirals; 5) weak and patchy; 6) leafy vines.                                                                                                                                                                                                                                                      |
|       | 13-16 | **Unusual odour**. Your body emits an unusual, very noticeable odour. D6: 1) musk; 2) sulphur; 3) dung; 4) rotten flesh; 5) ammonia; 6) rusted iron.                                                                                                                                                                                                                                                                                                                                                          |
|       | 17-20 | **Unusual voice**. Your voice changes in an unusual way. D6: 1) raspy whisper; 2) hissing; 3) high pitched; 4) powerful growl; 5) buzzing as if produced by insects; 6) unrecognisable accent.                                                                                                                                                                                                                                                                                                                |
| 9-10  |  1-2  | **Lost body part**. One part of your body disappears, potentially bringing similar disadvantages as an injury. D6: 1) an arm; 2) a leg; 3) an eye; 4) an ear; 5) your nose; 6) your teeth.                                                                                                                                                                                                                                                                                                                    |
|       |  3-4  | **Extra body part**. An additional body part sprouts somewhere on your body. D6: 1) an arm; 2) a leg; 3) an eye; 4) an ear; 5) a nose; 6) a mouth.                                                                                                                                                                                                                                                                                                                                                            |
|       |  5-6  | **Re-arranged body parts**. Some body parts swap their positions, for example your legs with your arms, or your eyes with your ears.                                                                                                                                                                                                                                                                                                                                                                          |
|       |  7-8  | **Weird blood**. Your blood is replaced by some weird substance. D6: 1) acid; 2) insects; 3) mud; 4) resin; 5) wine; 6) glue.                                                                                                                                                                                                                                                                                                                                                                                 |
|       | 9-10  | **Bestial legs**. D6: 1) goat legs; 2) grasshopper legs (can jump farther away); 3) spider legs (can climb sheer surfaces easily); 4) bird legs; 5) lizard legs; 6) fish tail (can swim at full speed, halves movement speed on land).                                                                                                                                                                                                                                                                        |
|       | 11-12 | **Bestial arm**. D6: 1) tentacle (your grapple attacks can't be saved against); 2) crab pincer (d6 damage); 3) sharp claws (d4 damage); 4) wing (not functional); 5) gorilla arm (d4 damage, STR permanently increased by 1); 6) mantis scythe (d6 damage).                                                                                                                                                                                                                                                   |
|       | 13-14 | **Bestial head**. D6: 1) horned goat head (horns: d4 damage); 2) fly head; 3) bird head (beak: d4 damage); 4) snake head (bite: d4 damage, poison: damaging); 5) bat head (echolocation); 6) frog head (sticky tongue with range 1, can grab items and characters of bulk 1 or less).                                                                                                                                                                                                                         |
|       | 15-16 | **Bestial skin**. D6: 1) thick fur; 2) bird feathers; 3) lizard scales; 4) fish scales; 5) elephant skin; 6) turtle shell (armour value 1).                                                                                                                                                                                                                                                                                                                                                                   |
|       | 17-18 | **Bestial tail**. D6: 1) scorpion (d6 damage, paralysing poison on 1 or more damage); 2) monkey (prehensile); 3) goat; 4) lizard (regrows if cut); 5) pig; 6) dog (it wiggles when you are excited);                                                                                                                                                                                                                                                                                                          |
|       | 19-20 | **Bestial wings**. You gain the _flying (lander)_ trait. D4: 1) falcon; 2) butterfly; 3) fly; 4) bat.                                                                                                                                                                                                                                                                                                                                                                                                         |
|  11   |   1   | **Bone spurs**. Sharp bone spurs grows all over your body. They can be used as a weapon inflicting d6 damage.                                                                                                                                                                                                                                                                                                                                                                                                 |
|       |   2   | **Elastic limbs**. Your arms, legs, and neck are elastic and can extend up to range 1.                                                                                                                                                                                                                                                                                                                                                                                                                        |
|       |   3   | **Detachable limbs**. You suffer no bleeding when body parts are cut away from you, and you can still move them as if they were still connected to your body. Reconnecting them to your body requires stitching.                                                                                                                                                                                                                                                                                              |
|       |   4   | **Acid secretion**. You secrete an extremely acid substance. Any piece of clothing or armour you wear is destroyed within seconds, and you inflict d4 damage per round to characters you grab.                                                                                                                                                                                                                                                                                                                |
|       |   5   | **Foul secretion**. You secrete a foul, smelly substance. Everyone you touch is exposed to a disease. D6: 1) influence; 2) bloody influence; 3) plague; 4) black gangrene; 5) bloodburn; 6) weeping sores.                                                                                                                                                                                                                                                                                                    |
|       |   6   | **Skull face**. Your face turns into a skull. You gain the _frightening_ trait.                                                                                                                                                                                                                                                                                                                                                                                                                               |
|       |   7   | **Darksight**. You can see in darkness, but in daily light you can see at most up to range 1.                                                                                                                                                                                                                                                                                                                                                                                                                 |
|       |   8   | **Corrosive vomit**. Once per daily rest, you can vomit a large amount of corrosive liquid. This is an attack inflicting d6 damage and damaging the equipment of your target.                                                                                                                                                                                                                                                                                                                                 |
|       |   9   | **Fire breath**. Once per daily rest, you can breathe a stream of fire. This is a blast attack inflicting d6 fire damage.                                                                                                                                                                                                                                                                                                                                                                                     |
|       |  10   | **Swarm of insects**. You are constantly accompanied by a swarm of flies, maggots, and cockroaches, swarming around you and molesting anyone trying to harm you. Enemies attempting to attack you in melee must pass a STR save to be able to hit you.                                                                                                                                                                                                                                                        |
|       |  11   | **Evil eye**. One of your eyes turns black like the night. You gain the _sorcery_ skill, or your maximum mana is increased by 1 if you already have the skill. Your sight isn't affected.                                                                                                                                                                                                                                                                                                                     |
|       |  12   | **Eye of the prophet**. One of your eyes turn milky white like a pearl. Your maximum omens increase by 1. Your sight isn't affected.                                                                                                                                                                                                                                                                                                                                                                          |
|       |  13   | **Boneless**. Your skeleton leaves your body to live its own life as an undead and you become a soft, boneless organism. Your STR and AGI are both permanently reduced by d4, but you are able to bend your body in impossible ways.                                                                                                                                                                                                                                                                          |
|       |  14   | **Skeleton**. Your skin and muscles turn into dust, leaving you as a bloody skeleton surrounding your internal organs. You gain the _undead_ and _frightening_ traits, but your STR and AGI are permanently reduced by d4.                                                                                                                                                                                                                                                                                    |
|       |  15   | **Flying head**. Your head detaches from your body, and from now on you must live as a bodiless flying head. You gain the _flying (hoverer)_ trait and your STR is permanently halved, besides any other obvious effects.                                                                                                                                                                                                                                                                                     |
|       |  16   | **Mouth of truth**. A fanged mouth opens on your neck, which reveals your secrets at the least opportune moments. It is silenced for a day if it drinks human blood.                                                                                                                                                                                                                                                                                                                                          |
|       |  17   | **Sentient tumour**. A cancerous lump grows somewhere on your body. It has its own alien intellect and constantly tries to take control over you. The GM makes a secret WIT roll for you at the start of each day: on a fail, the tumour takes control over your body for a stretch during the day. When and how is decided by the GM. When you die, the tumour detaches from your body and turns into a tiny copy of you, which grows to full size after a week, potentially taking your place in the world. |
|       |  18   | **Evil twin**. A lump detaches from your body and grows into an identical twin, who is however extremely evil and will try to soil your good name in any way possible.                                                                                                                                                                                                                                                                                                                                        |
|       |  19   | **Demonic nature**. You feel that something is different, as if you didn't belong to this world any more. You gain the _demon_ trait.                                                                                                                                                                                                                                                                                                                                                                         |
|       |  20   | **Uncanny resemblance**. Your appearances changes to match that of another person.                                                                                                                                                                                                                                                                                                                                                                                                                            |
|  12   | 1-20  | **Roll twice**.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |


## Omens

Omens represent luck and divine favour.
Player characters start with 2 omens, the maximum they can accumulate.
Most GM characters have no omens, but particularly important ones might have 1 or 2.
Omens can be spent in the following ways:

* Re-roll a die, keeping the most favourable results: a failed save, a lost contest, damage rolls, soulblight checks, chance rolls (x:y chances), rolls on a table.
It is possible to spend multiple omens on the same roll.

* Save from death, either due to health loss or instant-death effects.
Health is set to 1 and the character is _incapacitated_ until the end of the stretch, but they clear the _dying_ condition if they have it.
Finally, they must roll for an injury (as if they recovered from the _dying_ condition), however in this case the effect is permanent!

* Invoke sacred powers (see the [Magic rules]({{< relref "magic_rules.md" >}}) chapter).

Spent omens are recovered on a full rest, but the GM may also reward an omen when the Players do something which their deity or a powerful magic creature approve of.

{{% details title="Example" closed="true" %}}

Theobald (STR 9) is hit by a poisonous dart and must pass a STR save to resist the toxin.
He rolls a 10, a failure, but decides to spend an omen to re-roll.
This time he gets a 1: a success!

Balthasar (AGI 10), Sybilla (AGI 10), and Theobald (AGI 6) are racing against each other, and must resolve it as an AGI contest.
Balthasar rolls an 8, Sybilla a 19, and Theobald a 2.
The ranking is: Theobald (6 − 2 = 4), Balthasar (10-8 = 2), Sybilla (10 - 19 = -9).
Sybilla decides to spend an omen and rolls 7, for a new total of: 10 - 7 = 3.
She is now second after Theobald.

Sybilla suffers d4 damage.
She rolls 4 (the maximum), then 2, for a total of 5 damage.
She decides to spend an omen to re-roll the damage.
This time, she rolls a 2, so she only suffers 2 damage.

Balthasar suffers 4 corruption, bringing the total to 4, and rolls a d12 to check for soulblight.
He gets a 4, which would mean 4 direct damage.
He spends an omen to re-roll and this time gets a 5.
He decides it's a better result, so he keeps it and suffers no damage.

Sybilla (STR 8, current health 6) suffers 10 damage, which would be enough to kill her on the spot.
She spends an omen to save herself: her health is set to 1, and she is _incapacitated_.
She rolls a 6 on the Injuries table, meaning she has permanently lost an eye.

{{% /details %}}


## Death of Player characters

Players whose character dies must immediately create a new one, who joins the company as soon as possible under whatever narrative pretext can be devised by the GM.
If all Player characters are _incapacitated_ at the same time, the GM decides what happens to the company (they might be killed, captured, rescued, robbed, etc.).
If all Player characters die at the same time, the Players lose the game and must create a brand-new company.

When a Player character dies, the other members of the company must pay for a funeral at the latest until their next full rest.
It costs 512ʂ, halved if the dead character's remains are returned and given proper rest.
Failing to pay for a funeral brings bad luck: all other Player characters permanently reduce their maximum omens by 1.
If they already have 0 omens, they suffer no additional penalties.


## Money

Money is measured in shillings (ʂ).
You should track it down to a quarter shilling.
The rules provide indicative costs for items, followers, and services, which the GM can alter as they see fit.
Goods in short supply are typically worth double, while goods in oversupply are worth half.
Damaged goods are worth nothing.


## Daily rests

Characters can spend a watch resting to recover their energies and restore health and mana to the maximum.
On the third watch, if they don't rest, they they suffer from the _groggy_ condition, as they start to get hungry, tired, and sleepy.
On the fourth watch, they are too exhausted to do anything else and are forced to take a daily rest.

Characters who take a daily rest must satisfy their daily need for food, water, and good sleep.
If they fail to do so, they recover no health nor mana, and additionally temporarily reduce an ability by 1: STR if they didn't eat, AGI if they didn't drink, and WIT if they didn't sleep well.

**Food**.
Consume a _ration_ or purchase a meal in a settlement for 2ʂ.
Looking for food in the wilderness takes a watch: roll on the foraging table below to see if something edible is found.
_Rations_ gathered in this way are perishable and are discarded if not consumed on the next daily rest.

**Water**.
Consume a unit of _water_.
While in a settlement or another environment with plenty of clean, drinkable water, you don't need to track this need.
If water is scarce, finding a source takes a watch and requires passing a WIT save.

**Sleep**.
Use a _camping kit_ or rent a room in a settlement for 2ʂ.

Animal followers don't need a _camping kit_ to sleep, and they don't need a _ration_ or a _drink_ while in their natural habitat, as they can forage for themselves.
In a settlement, you have to pay 2ʂ for food and 2ʂ for room and care in a stable.

|  D6   | Foraging result                                                                                                                                                                                                            |
| :---: | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|   1   | **Choice**. Pick any one of the following entries.                                                                                                                                                                         |
|   2   | **Fruits and vegetables**. You find a _ration_ unless you are in an environment devoid of vegetation. If consumed, there is a 1:8 chance that it exposes you to a disease, poison, drug, or other effect chosen by the GM. |
|   3   | **Fish**. You find a _ration_ if you have a _fishing kit_ and there is a body of water in your region.                                                                                                                     |
|   4   | **Small game**. You find a _ration_ if you have a _trapping kit_ and you are in an environment where small game can be found.                                                                                              |
|   5   | **Big game**. If you are in an environment where large game can be found, you find 4 _rations_ if you pass a WIT save to track a prey and then roll 4 or more damage with a ranged attack to kill it.                      |
|   6   | **Nothing**. You come back empty-handed.                                                                                                                                                                                   |


## Full rests

Characters can spend a week resting to recover health, mana, and omens to the maximum, restore ability scores to their original value, and and get rid of temporary injuries.
They can also perform a single full rest activity, which can be any action which is described as such or which takes a watch.

It is only possible to take a full rest in a settlement, not when camping in the wilderness.
It costs 32ʂ, plus 16ʂ for each temporary condition which is cleared by the full rest.
The price includes food, water, a place to sleep, medical care, and entertainment.


## Encumbrance

Characters can carry up to 8 bulk unencumbered, and up to twice that _encumbered_.
_Encumbered_ characters move at half speed and must pass an additional STR save each time they make an AGI save.

* An item's bulk equals 1, unless otherwise indicated in its description.

* A character's bulk equals their carry limit: 8 for most characters.

* Groups of 32ʂ have bulk equal to ½.
31 or fewer shillings can be carried freely.

Characters must have a reasonable way to store what they are carrying.
If they are wearing _clothes_, they are assumed to have enough containers (backpacks, pouches, pockets, scabbards, etc.) to store any amount of items bulk 2 or less.

{{% details title="Example" closed="true" %}}

Sybilla (STR 8, AGI 10) is carrying a sword (bulk 1), a bow (bulk 2), light armour (bulk 2), clothes (bulk 1), 64ʂ (bulk 2), and a camping kit (bulk 2).
This totals 10 bulk, meaning she is _encumbered_.
She attempts to leap across a chasm, which requires an AGI save.
She rolls a 9, which would normally be a pass, but must also pass a STR save since she is _encumbered_.
This time she rolls an 11, a fail, meaning that she falls into the chasm!

{{% /details %}}


## Durability rolls

Each time an item is used in a way which might compromise its integrity, there is a 1:4 chance it is _damaged_.
Only roll if you are unsure if the item was _damaged_: if it's pretty clear it was _damaged_ or destroyed, it just happens.
_Damaged_ items can still be used, but are permanently destroyed on another failed durability roll.

{{% details title="Example" closed="true" %}}

Balthasar uses a sword to try to break down a door.
After the attempt, the GM rules that this might have ruined the blade, and therefore asks for a durability roll.
Balthasar rolls a 1 on a d4, meaning that the sword was indeed _damaged_.

Sybilla violently throws a glass jar at the ground.
The glass just shatters, no durability roll required.

{{% /details %}}


## Followers

Players characters can be accompanied by up to 4 followers, whom they control as their own character, assuming they are giving them orders.
However, the GM ultimately decides if and how they do what they are asked, as they act in their own best interest.
In situations where their loyalty is put into question, the character who is leading them must pass a WIT save to keep their allegiance.

**Retainers**.
Followers of human intelligence act as retainers, joining the party due to common interests or in exchange for payment.
They pretty much behave like Player characters, but might refuse to perform tasks outside of their agreed responsibilities.

**Animal followers**.
Animal followers behave according to their nature and instincts.
If trained, they might be able to obey simple orders and perform specific tasks, such as carrying a rider, pulling carts, following trails, performing tricks, etc.
Animals will avoid combat unless they are natural predators or have been trained to fight.


## Maps

The scenario should provide maps illustrating the locations where the adventure takes place.
During play, the GM can copy the maps on paper as the Players explore locations, gradually revealing their layout and features.
Maps are divided into areas.
Just like time is tracked using three time units, the game uses three types of areas: zones, sectors, and regions.

**Zones** are used in maps representing buildings, dungeons, and other small sites.
A zone covers an area of about 4 metres by 4 metres, such as a small room, a portion of a large room, a corridor, etc.

**Sectors** are used in maps representing towns, districts, and other large sites.
A sector covers an area of about 400 metres by 400 metres, such as a city district, a large building, a garden, etc.

**Regions** are used in maps representing vast regions.
A region covers an area of about 10 kilometres by 10 kilometres, such as a whole city, a portion of a forest, a mountain pass, etc.

Areas don't have to be all of the same shape (such as squares or hexagons) or size.
As a matter of fact, the GM can slightly vary the size and shape of areas to represent obstacles to movement.
For example, mountainous lands might be divided in smaller regions when compared to grassy plains, to represent the increased effort and time required to cross them.


## Positioning

Characters can be located in an area (zone, sector, or region) or on the border between two areas.
At most up to 8 characters can fit inside a zone, and up to 2 characters can fit on the border between two zones.
This amount is halved for tiny and cramped zones, or doubled for large and open zones.
Borders representing a door or similar small passages can typically hold only a single character.
Sectors and regions don't have occupancy limits, since they represent a much larger scale.


## Movement

Characters can move by two areas per time unit: two zones in a round, two sectors in a stretch, or two regions in a watch.
Moving to or from the border between two areas counts as moving by half an area.

Characters can move through a full zone or zone border which is mostly occupied by allies, but can't end the movement on it.
They can't move through a full zone or zone border which is mostly occupied by enemies.


## Range

Ranged attacks, powers, and other effects have a range expressed in number of zones.
To determine if two zones are within range, find the shortest path between them and count how many zones it crosses, excluding the originating one.

Characters on a border are considered to be in either zone for the purpose of range.
This means that characters in a zone are at range 0 from all characters in the same zone and on all its borders, and characters on a border are at range 0 from other characters on the same border or in either of the two zones it divides.
Characters at range 0 from each other are said to be nearby and can be targeted by melee attacks and other short range effects.

