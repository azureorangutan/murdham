import * as Data from "./data.js"

const light_weapons = [
    "cudgel",
    "dagger",
    "knife",
    "knuckledusters",
    "garrotte",
    "parrying dagger",
];

const simple_hand_weapons = [
    "bullwhip",
    "cleaver",
    "club",
    "hammer",
    "hatchet",
    "sickle",
];

const simple_great_weapons = [
    "boat hook",
    "hoe",
    "large club",
    "pickaxe",
    "pitchfork",
    "quarterstaff",
    "sledgehammer",
    "shovel",
    "spade",
    "woodcutting axe",
];

const martial_hand_weapons = [
    "arming sword",
    "battleaxe",
    "flail",
    "mace",
    "morningstar",
    "spear",
    "thrusting sword",
    "warhammer",
];

const martial_great_weapons = [
    "glaive",
    "halberd",
    "heavy battleaxe",
    "heavy flail",
    "lucerne hammer",
    "pike",
    "voulge",
    "zweihänder",
];

const missile_hand_weapons = [
    "blowgun",
    "darts",
    "hand crossbow",
    "harpoons",
    "javelins",
    "sling",
    "throwing knives",
    "throwing stars",
];

const missile_great_weapons = [
    "bow",
    "crossbow",
];

const handguns = [
    "dragon",
    "pistol",
];

const long_guns = [
    "arquebus",
    "blunderbuss",
];

function random_array_element(array) {
    let idx = Math.floor(Math.random() * array.length);
    return array[idx];
}

function ability_arrays() {
    let sum = 22;
    let min = 5;
    let max = 12;
    let values = []
    for (let a = min; a <= max; ++a) {
        for (let b = a; b <= max; ++b) {
            for (let c = b; c <= max; ++c) {
                if (a + b + c == sum) {
                    values.push([a, b, c]);
                }
            }
        }
    }
    return values;
}

function shuffle(arr) {
    let i = arr.length;
    while (--i > 0) {
        let j = Math.floor(Math.random() * (i + 1));
        let temp = arr[j];
        arr[j] = arr[i];
        arr[i] = temp;
    }
}

function random_abilities() {
    const arrays = ability_arrays();
    let array = random_array_element(arrays);
    shuffle(array);
    return array;
}

function random_career() {
    return random_array_element(Data.careers)
}

function random_age() {
    return 5 + 10 * Math.floor(Math.random() * 4 + 1) + Math.floor(Math.random() * 10 + 1);
}

function generate_character() {
    // Abilities
    let [str, agi, wit] = random_abilities();

    // Career
    let career = random_career();

    // Career advancements
    str += career.str;
    agi += career.agi;
    wit += career.wit;
    let skills = [...career.skills];
    let trappings = [...career.trappings];

    // Advancement
    switch (1 + Math.floor(Math.random() * 4)) {
        case 1:
            str += 1;
            agi += 1;
            break;
        case 2:
            str += 1;
            wit += 1;
            break;
        case 3:
            agi += 1;
            wit += 1;
            break;
        case 4:
            let new_skill = random_array_element(Data.starting_skills);
            while (skills.includes(new_skill)) {
                new_skill = random_array_element(Data.starting_skills);
            }
            skills.push(new_skill);
            break;
    }

    // Extra item.
    let extra_item = random_array_element(Data.extra_items);
    if (extra_item.includes("poison")) {
        let poison_type = "";
        switch (1 + Math.floor(Math.random() * 3)) {
            case 1:
                poison_type = "bloodstream";
                break;
            case 2:
                poison_type = "inhaled";
                break;
            case 3:
                poison_type = "ingested";
                break;
        }
        extra_item += " (" + poison_type + ")";
    }
    else if (extra_item == "light weapon") {
        let weapon = random_array_element(light_weapons);
        extra_item += " (" + weapon + ")";
    }
    else if (extra_item == "simple hand weapon") {
        let weapon = random_array_element(simple_hand_weapons);
        extra_item += " (" + weapon + ")";
    }
    else if (extra_item == "simple great weapon") {
        let weapon = random_array_element(simple_great_weapons);
        extra_item += " (" + weapon + ")";
    }
    else if (extra_item == "martial hand weapon") {
        let weapon = random_array_element(martial_hand_weapons);
        extra_item += " (" + weapon + ")";
    }
    else if (extra_item == "martial great weapon") {
        let weapon = random_array_element(martial_great_weapons);
        extra_item += " (" + weapon + ")";
    }
    else if (extra_item == "missile hand weapon") {
        let weapon = random_array_element(missile_hand_weapons);
        extra_item += " (" + weapon + ")";
    }
    else if (extra_item == "missile great weapon") {
        let weapon = random_array_element(missile_great_weapons);
        extra_item += " (" + weapon + ")";
    }
    else if (extra_item == "handgun") {
        let weapon = random_array_element(handguns);
        extra_item += " (" + weapon + ")";
    }
    else if (extra_item == "long gun") {
        let weapon = random_array_element(long_guns);
        extra_item += " (" + weapon + ")";
    }
    let money = trappings[trappings.length - 1]
    trappings[trappings.length - 1] = extra_item;
    trappings.push("waterskin");
    trappings.push(money);

    // Spells
    let used_spells = [];
    for (let i = 0; i < trappings.length; ++i) {
        if (trappings[i] == "arcane scroll") {
            while (true) {
                let spell = random_array_element(Data.arcane_spells);
                if (!used_spells.includes(spell)) {
                    trappings[i] = "arcane scroll (" + spell + ")";
                    used_spells.push(spell);
                    break;
                }
            }
        }
        if (trappings[i] == "sacred scroll") {
            while (true) {
                let spell = random_array_element(Data.sacred_spells);
                if (!used_spells.includes(spell)) {
                    trappings[i] = "sacred scroll (" + spell + ")";
                    used_spells.push(spell);
                    break;
                }
            }
        }
    }
    for (let i = 0; i < skills.length; ++i) {
        if (skills[i] == "witchcraft") {
            while (true) {
                let spell = random_array_element(Data.arcane_spells);
                if (!used_spells.includes(spell)) {
                    skills[i] = "witchcraft (" + spell + ")";
                    used_spells.push(spell);
                    break;
                }
            }
        }
        if (skills[i] == "saint") {
            while (true) {
                let spell = random_array_element(Data.sacred_spells);
                if (!used_spells.includes(spell)) {
                    skills[i] = "saint (" + spell + ")";
                    used_spells.push(spell);
                    break;
                }
            }
        }
    }

    // Description
    let age = random_age();
    let gender = random_array_element(Data.genders);
    let goal = random_array_element(Data.goal);
    let appearance = random_array_element(Data.appearance);
    let personality = random_array_element(Data.personality)

    let first_name = undefined;
    if (gender == "Male") {
        first_name = random_array_element(Data.masculine_first_names);
    }
    else {
        first_name = random_array_element(Data.feminine_first_names);
    }
    let last_name = random_array_element(Data.last_names);

    // Derived stuff
    let trappings_map = {};
    let trappings_map_len = 0;
    for (trapping of trappings) {
        if (trapping in trappings_map) {
            trappings_map[trapping]++;
        }
        else {
            trappings_map[trapping] = 1;
            trappings_map_len++;
        }
    }

    let trappings_str = "";
    let counter = 0;
    for (const [trapping, count] of Object.entries(trappings_map)) {
        if (count > 1) {
            trappings_str += count + "× ";
        }
        trappings_str += trapping;
        counter++;
        if (counter < trappings_map_len) {
            trappings_str += ", ";
        }
    }

    let health = str;
    if (skills.includes("tough")) {
        health += 3;
    }

    let omens = 2;
    if (skills.includes("lucky")) {
        omens += 1;
    }

    let mana = 0;
    if (skills.includes("sorcery")) {
        mana += 1;
    }

    let career_name = gender == "Male" ? career.male_name : career.female_name;
    document.getElementById("name").innerHTML = first_name + " " + last_name + " the " + career_name;
    document.getElementById("description").innerHTML =
        "<i>" +
        gender + ", " + age + " years old." +
        " " + career.description +
        " " + goal +
        " " + appearance +
        " " + personality +
        "</i>";
    document.getElementById("str").innerHTML = str
    document.getElementById("agi").innerHTML = agi
    document.getElementById("wit").innerHTML = wit
    document.getElementById("health").innerHTML = health;
    document.getElementById("omens").innerHTML = omens;
    document.getElementById("mana").innerHTML = mana;

    document.getElementById("skills").innerHTML = skills.join(", ");
    document.getElementById("trappings").innerHTML = trappings_str;
}

document.getElementById("chargen_button").onclick = generate_character;
generate_character();
