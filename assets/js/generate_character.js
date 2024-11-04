import * as Data from "./data.js"

function random_array_element(array) {
    let idx = Math.floor(Math.random() * array.length);
    return array[idx];
}

function ability_arrays() {
    let min = 5;
    let max = 14;
    let values = []
    for (let a = min; a <= max; ++a) {
        for (let b = min; b <= max; ++b) {
            for (let c = min; c <= max; ++c) {
                if (a + b + c == 24) {
                    values.push([a, b, c]);
                }
            }
        }
    }
    return values;
}

function random_abilities() {
    const arrays = ability_arrays();
    return random_array_element(arrays);
}

function random_money() {
    return Math.floor(Math.random() * 8 + 1);
}

function random_age() {
    return 5 + 10 * Math.floor(Math.random() * 4 + 1) + Math.floor(Math.random() * 10 + 1);
}

function generate_character() {
    let [str, agi, wit] = random_abilities();
    let background = random_array_element(Data.backgrounds);
    let [weapon, money1] = random_array_element(Data.starting_weapons);
    let [item, money2] = random_array_element(Data.starting_items);
    let money3 = random_money();

    let age = random_age();
    let gender = random_array_element(Data.genders);
    let reasons_to_adventure = random_array_element(Data.reasons_to_adventure);
    let appearance = random_array_element(Data.appearance);
    let personality = random_array_element(Data.personality)

    let first_name = undefined;
    let background_name = undefined;
    if (gender == "Male") {
        first_name = random_array_element(Data.masculine_first_names);
        background_name = background.masculine_name;
    }
    else {
        first_name = random_array_element(Data.feminine_first_names);
        background_name = background.feminine_name;
    }
    let last_name = random_array_element(Data.last_names);

    let items = [weapon, item];
    for (let i = 0; i < background.items.length; i++) {
        items.push(background.items[i]);
    }
    for (let i = 0; i < background.profane_scrolls; i++) {
        while (true) {
            let new_scroll = "power scroll (" + random_array_element(Data.profane_powers) + ")";
            if (!items.includes(new_scroll)) {
                items.push(new_scroll);
                break;
            }
        }
    }
    for (let i = 0; i < background.sacred_scrolls; i++) {
        while (true) {
            let new_scroll = "power scroll (" + random_array_element(Data.sacred_powers) + ")";
            if (!items.includes(new_scroll)) {
                items.push(new_scroll);
                break;
            }
        }
    }

    let items_map = {};
    for (item of items) {
        if (item in items_map) {
            items_map[item]++;
        }
        else {
            items_map[item] = 1;
        }
    }

    let items_str = "";
    for (const [item, count] of Object.entries(items_map)) {
        if (count > 1) {
            items_str += count + "× ";
        }
        items_str += item + ", ";
    }

    let money = money1 + money2 + money3 + background.money;

    document.getElementById("name").innerHTML = first_name + " " + last_name + " the " + background_name;
    document.getElementById("description").innerHTML =
        gender + ", " + age + " years old. " +
        background.description +
        " You have abandoned your previous life because " + reasons_to_adventure +
        " " + appearance +
        " " + personality;
    document.getElementById("str").innerHTML = str
    document.getElementById("agi").innerHTML = agi
    document.getElementById("wit").innerHTML = wit
    document.getElementById("mana").innerHTML = background.mana;
    if (background.mana == 0) {
        document.getElementById("mana-display").style.display = 'none';
    }
    else {
        document.getElementById("mana-display").style.display = 'inline';
    }

    document.getElementById("skills").innerHTML = background.skills.join(", ");
    document.getElementById("items").innerHTML = items_str + money + "ʂ";
    if (background.followers.length > 0) {
        document.getElementById("followers_container").style.display = "block"
        document.getElementById("followers").innerHTML = background.followers.join(", ");
    }
    else {
        document.getElementById("followers_container").style.display = "none"
    }
}

document.getElementById("chargen_button").onclick = generate_character;
generate_character();
