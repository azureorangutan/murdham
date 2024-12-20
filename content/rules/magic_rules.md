---
title: Magic Rules
weight: 100
prev: /rules/combat_rules
next: /rules/bestiary
---

## Power scrolls

_Power scrolls_ are precious artefacts holding ancient and powerful magic.
They come in two types: sacred and profane.

Characters with the _faith_ or _sorcery_ skill can invoke powers from sacred and profane _power scrolls,_ respectively.
They must hold the _power scroll_ while reading it out loud, just memorizing the incantations isn't enough.

Knowledge on how to craft power scrolls has been long forgotten.
Simply transcribing the incantations on parchment doesn't work.
The Church toils to collect and hoard sacred scrolls, and to burn and destroy profane scrolls.


## Invoking sacred powers

Invoking a sacred power takes a full stretch and consumes an omen.
The invoker must roll higher than their current corruption on a d12 for the power to work, otherwise it has no effect.

{{% details title="Example" closed="true" %}}

Norman the Priest has the _faith_ skill and a _power scroll_ recording the _rite of martyrdom_.
He can attempt to invoke this sacred power by spending an omen and a stretch.
He currently has 2 corruption, meaning that the power is only effective on a roll of 3 or more on a d12.

{{% /details %}}


## Invoking profane powers

Invoking a profane power only takes a round.
The invoker must spend 1 mana to cast the power, or suffer 2 corruption if that's not possible.

Invokers can spend a number of enhancement points (EPs) equal to their current mana, before reducing it, to power up a profane power.
They can get an additional EP by spending a full stretch instead of a round to invoke the power, by suffering 2 corruption, or by consuming a _dark essence_.
Each can be done only once, but all three can be done together to gain 3 additional EPs.

EPs can be spent to buy the following generic enhancements, or any specific enhancement specified in the power's description:

* 1 EP -- Double the number of targets.

* 1 EP -- Improve range by one category.

* 2 EP -- Improve duration by one category.

* 1 EP -- Ignore the effects of the _magic shield_ skill.

{{% details title="Example" closed="true" %}}

Philippa has the _sorcery_ skill and two _power scrolls_ recording the _eldritch blast_ and _miasma of chaos_ spells.
She currently has 2 mana, meaning she can cast two spells safely.

She first casts _eldritch blast_ with 2 EPs (her current mana), choosing to increase range by a category and doubling the number of targets.
The power takes effect and her mana is reduced to 1.

Later, she casts _eldritch blast_ again, this with only 1 EP.
She decides to suffer 2 corruption to get a second EP and apply the same enhancements as before.
Luckily she suffers no soulblight, the power works, and her mana is reduced to 0.

Later she casts _miasma of chaos_.
Since she has no mana left, she suffers 2 corruption, increasing her total to 4.
Moreover, she has no free EPs, but could still decide to get additional EPs by suffering corruption, taking longer to invoke the power, or consuming a _dark essence_.
She decides against it, therefore casting the power in its basic form.

{{% /details %}}


## Power range

* **Touch**.
Targets must be nearby and be physically touched.

* **Sight**.
Targets must be visible and within range 8.

* **Connection**.
Targets can be anywhere as long as the invoker has an arcane connection to them.
An arcane connection is formed through an object which is closely related to the target (a lock of hair, a personal possession, etc.), which is destroyed upon invoking the power.

Powers with _touch_ or _sight_ range are considered melee or ranged attacks, respectively.
All relevant skills and rules apply, as long as it makes sense.
In particular, targets can react to powers, and powers can be invoked to counter an attack, assuming the power takes a round to invoke.
Powers with _connection_ range aren't handled as attacks.

Powers can always be cast at a worse range: for example, a power with _connection_ range can be cast with _sight_ or _touch_ range.
Furthermore, invokers can always target themselves, no matter the range.


## Power duration

* **Stretch**.
The power lasts until the end of the stretch.

* **Watch**.
The power lasts until the end of the watch.

* **Lingering**.
The power lasts indefinitely.

Some powers might end prematurely based on particular conditions.
Upon casting, the invoker may also choose a trigger which can end the power prematurely (a word, a gesture, an event, etc.).

Ongoing profane powers can additionally be dispelled by casting the exact same power on the same target.
By default, it's only possible to dispel a power with _stretch_ duration in this way.
By spending 2 EP or 4 EP, it is possible to dispel a _watch_ or _lingering_ power respectively.


## Magic corruption and miscasts

As previously mentioned, it is possible to invoke a profane power with no mana left and/or with an additional EP by suffering 2 corruption.
If both conditions hold true, the invoker suffers 4 corruption and only checks for soulblight once.

Corruption is inflicted before the power takes effect.
If the caster resists soulblight, the power works flawlessly.
If they suffer soulblight and are killed or _incapacitated_, the power has no effect.
If they suffer soulblight but are still alive and conscious, the power works, but they must roll on the miscast table below.

{{% details title="Example" closed="true" %}}

Philippa attempts to cast _eldritch blast_ while having 0 mana and 4 corruption.
Her corruption increases to 6, and she rolls 3 on a d12, causing soulblight!
Her corruption is reduced to 3 and her health is reduced from 8 to 5, so she is still conscious.
The power works, but Philippa must roll on the miscast table.
She rolls an 8, meaning the _eldritch blast_ has an alternative effect.
The GM rules that the power heals the target instead of damaging them!

Later, Philippa casts _eldritch blast_ again but this time with 1 EP, therefore suffering 4 corruption at once since her mana is still 0.
Her total corruption increases to 7, and she rolls a 4 on a d12, meaning she suffers soulblight again!
This time her health is reduced from 5 to 1, so she suffers critical damage: she is _incapacitated_ and suffers a mutation.
Since she was _incapacitated_, the power has no effect.

{{% /details %}}

|  D12  | Miscast                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :---: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
|   1   | **Fizzle**. The power actually doesn't work, but there are no other penalties.                                                                                                                                                                                                                                                                                                                                                             |
|   2   | **Eldritch event**. A minor eldritch event happens around the invoker: plants wither, blood rains from the sky, the air becomes unnaturally cold, etc. This might spook other people, but otherwise has no significant consequences. Anyone witnessing the event must pass a group WIT save or become _frightened_.                                                                                                                        |
|   3   | **Darkness**. Thick magical darkness or fog shrouds all zones within range 1 of the invoker until the end of the watch. It's impossible to see through it, and light cannot penetrate it in any way.                                                                                                                                                                                                                                       |
|   4   | **Lost power**. The _power scroll_ burns or crumbles to dust and is permanently destroyed.                                                                                                                                                                                                                                                                                                                                                 |
|   5   | **Corruption**. The floor, walls, and air around the invoker twirl and change in chaotic patterns. All other characters within range 1 of the invoker suffer 2 corruption.                                                                                                                                                                                                                                                                 |
|   6   | **Explosion**. Black flames burst from the invoker. All other characters within range 1 of the invoker suffer d6 fire damage. They can attempt to dodge.                                                                                                                                                                                                                                                                                   |
|   7   | **Wrong target**. The power hits the wrong targets, in a way which is disadvantageous for the invoker. Re-roll if this doesn't make sense.                                                                                                                                                                                                                                                                                                 |
|   8   | **Alternative effect**. The power has an alternative effect which is disadvantageous for the invoker, typically the exact opposite of what it was intended to do. Re-roll if this doesn't make sense.                                                                                                                                                                                                                                      |
|   9   | **Demon pox**. The invoker and all characters within range 1 are exposed to the _demon pox_ disease.                                                                                                                                                                                                                                                                                                                                       |
|  10   | **Mutation**. The invoker develops a random mutation.                                                                                                                                                                                                                                                                                                                                                                                      |
|  11   | **Summoning**. A demon enters the world nearby the invoker. It isn't necessarily hostile, but its presence will surely spell trouble at some point.                                                                                                                                                                                                                                                                                        |
|  12   | **Possession**. A demon possesses the invoker. The GM makes a secret d12 roll for them at the start of each day. If they roll equal or lower than their corruption, the GM can take control of the character for a whole stretch at any point during the day, making them do the demon's bidding. Characters who are possessed a second time succumb to the demon's power, who takes full control of their body, and are effectively dead. |


## Sacred powers

{{% include "../gen/ref_sacred_powers.md" %}}


## Profane powers

{{% include "../gen/ref_profane_powers.md" %}}
