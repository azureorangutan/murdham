use convert_case::{Case, Casing};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, error::Error, fs::File, io::Write, path::Path};

fn capitalise_second(mut s: String) -> String {
    if let Some(r) = s.get_mut(1..2) {
        r.make_ascii_uppercase();
    }
    s
}

fn capitalise(mut s: String) -> String {
    if s.starts_with(char::is_alphabetic) {
        if let Some(r) = s.get_mut(0..1) {
            r.make_ascii_uppercase();
        }
        s
    } else {
        capitalise_second(s)
    }
}

fn lower_second(mut s: String) -> String {
    if let Some(r) = s.get_mut(1..2) {
        r.make_ascii_lowercase();
    }
    s
}

fn lower(mut s: String) -> String {
    if s.starts_with(char::is_alphabetic) {
        if let Some(r) = s.get_mut(0..1) {
            r.make_ascii_lowercase();
        }
        s
    } else {
        lower_second(s)
    }
}

fn count_map(assets: &[AssetWithDescr]) -> Vec<(AssetWithDescr, i32)> {
    let mut ans: Vec<(AssetWithDescr, i32)> = Vec::new();
    let mut j = 0;
    for i in 0..assets.len() {
        if j > 0 && ans[j - 1].0 == assets[i] {
            ans[j - 1].1 += 1;
        } else {
            ans.push((assets[i].clone(), 1));
            j += 1;
        }
    }
    ans
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum AttackProperty {
    Blast,
    Cold,
    DirectDamage,
    Fire,
    Grab,
    Heat,
    Lightning,
    Loud,
    OneShot,
    Piercing,
    Poison(String),
    Range(String),
    Reload,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Weapon {
    #[serde(default)]
    damage: Option<Die>,
    #[serde(default)]
    damage_descr: String,
    #[serde(default)]
    properties: Vec<AttackProperty>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum ItemProperty {
    Consumable,
    Addictive,
    Poison,
    Structure(i8, i8),
    Fragile,
    Robust,
    Weapon(Weapon),
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Item {
    name: String,
    #[serde(default)]
    value: i32,
    bulk: i32,
    #[serde(default)]
    properties: Vec<ItemProperty>,
    #[serde(default)]
    descr: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct AssetWithDescr {
    name: String,
    #[serde(default)]
    descr: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct AssetSet {
    #[serde(default)]
    items: Vec<AssetWithDescr>,
    #[serde(default)]
    followers: Vec<AssetWithDescr>,
    #[serde(default)]
    money: i8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Spell {
    name: String,
    descr: String,
    #[serde(default)]
    enhancements: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CharacterSize {
    Tiny,
    Medium,
    Huge,
}

impl Default for CharacterSize {
    fn default() -> Self {
        CharacterSize::Medium
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Trait {
    name: String,
    #[serde(default)]
    descr: String,
}

impl Markdownify for Trait {
    fn markdownify(&self) -> String {
        if self.descr.is_empty() {
            format!("{}", self.name)
        } else {
            format!("{} ({})", self.name, self.descr)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Character {
    name: String,
    #[serde(default)]
    cost: Option<i32>,
    #[serde(default)]
    descr: String,
    str: i8,
    agi: i8,
    wit: i8,
    #[serde(default)]
    mana: i8,
    #[serde(default)]
    omens: i8,
    #[serde(default)]
    armour: i8,
    #[serde(default)]
    traits: Vec<Trait>,
    #[serde(default)]
    special_traits: BTreeMap<String, String>,
    #[serde(default)]
    size: CharacterSize,
    #[serde(default)]
    attacks: Vec<(String, Weapon)>,
    #[serde(default)]
    items: Vec<AssetWithDescr>,
    #[serde(default)]
    money: i32,
}

trait Markdownify {
    fn markdownify(&self) -> String;
}

impl Markdownify for Die {
    fn markdownify(&self) -> String {
        match self {
            Self::D4 => "d4".to_string(),
            Self::D6 => "d6".to_string(),
            Self::D8 => "d8".to_string(),
            Self::D10 => "d10".to_string(),
            Self::D12 => "d12".to_string(),
        }
    }
}

impl Markdownify for AttackProperty {
    fn markdownify(&self) -> String {
        match self {
            Self::Range(x) => format!("{x} range"),
            _ => format!("{:?}", self).to_case(Case::Lower),
        }
    }
}

impl Markdownify for Weapon {
    fn markdownify(&self) -> String {
        let mut comps = Vec::new();
        if let Some(die) = self.damage {
            if self.damage_descr.is_empty() {
                comps.push(die.markdownify());
            } else {
                comps.push(format!("{} {}", die.markdownify(), self.damage_descr));
            }
        }
        for property in self.properties.iter() {
            comps.push(property.markdownify())
        }
        comps.join(", ")
    }
}

impl Markdownify for ItemProperty {
    fn markdownify(&self) -> String {
        match self {
            Self::Structure(x, y) => format!("structure {x}, armour {y}"),
            Self::Weapon(x) => format!("weapon ({})", x.markdownify()),
            _ => format!("{:?}", self).to_case(Case::Lower),
        }
    }
}

impl Markdownify for Item {
    fn markdownify(&self) -> String {
        let mut comps = Vec::new();

        let name = capitalise(self.name.clone());

        let mut cost_and_bulk = Vec::new();
        if self.value > 0 {
            cost_and_bulk.push(format!("{}ʂ", self.value));
        }
        if self.bulk > 1 {
            cost_and_bulk.push(format!("bulk {}", self.bulk));
        }

        let cost_and_bulk = if !cost_and_bulk.is_empty() {
            format!(" ({})", cost_and_bulk.join(", "))
        } else {
            String::new()
        };

        comps.push(format!("**{name}**{cost_and_bulk}."));

        if !self.properties.is_empty() {
            let properties = capitalise(
                self.properties
                    .iter()
                    .map(|x| format!("{}", x.markdownify()))
                    .collect::<Vec<String>>()
                    .join(", "),
            );
            comps.push(format!("{properties}."));
        }

        if !self.descr.is_empty() {
            let descr = capitalise(self.descr.clone());
            comps.push(descr);
        }

        comps.join("\n  ")
    }
}

impl Markdownify for AssetWithDescr {
    fn markdownify(&self) -> String {
        let mut ans = self.name.clone();
        if !self.descr.is_empty() {
            ans += &format!(" ({})", self.descr);
        }
        ans
    }
}

impl Markdownify for AssetSet {
    fn markdownify(&self) -> String {
        let mut comps = Vec::new();

        let items = count_map(&self.items);
        items.iter().for_each(|(item, count)| {
            comps.push(format!(
                "{}{}",
                if *count > 1 {
                    format!("{count}× ")
                } else {
                    String::from("")
                },
                item.markdownify()
            ));
        });

        let followers = count_map(&self.followers);
        followers.iter().for_each(|(follower, count)| {
            comps.push(format!(
                "{}{}",
                if *count > 1 {
                    format!("{count}× ")
                } else {
                    String::from("")
                },
                follower.markdownify()
            ));
        });

        if self.money > 0 {
            comps.push(format!("{}ʂ", self.money));
        }

        comps.join(", ")
    }
}

impl Markdownify for Spell {
    fn markdownify(&self) -> String {
        let mut comps = Vec::new();
        comps.push(format!("**{}**.", capitalise(self.name.clone())));
        comps.push(self.descr.clone());
        for enhancement in self.enhancements.iter() {
            comps.push(format!(
                "    * *Enhancement:* {}",
                lower(enhancement.clone())
            ));
        }
        comps.join("\n")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Career {
    male_name: String,
    female_name: String,
    #[serde(default)]
    str: i8,
    #[serde(default)]
    agi: i8,
    #[serde(default)]
    wit: i8,
    #[serde(default)]
    skills: Vec<String>,
    #[serde(default)]
    items: Vec<AssetWithDescr>,
    #[serde(default)]
    followers: Vec<AssetWithDescr>,
    #[serde(default)]
    money: i32,
    #[serde(default)]
    descr: String,
}

impl Markdownify for Career {
    fn markdownify(&self) -> String {
        let mut comps = Vec::new();

        comps.push(format!("*{}*", self.descr));

        {
            let mut abilities = Vec::new();
            if self.str > 0 {
                abilities.push(format!("+{} STR", self.str));
            }
            if self.agi > 0 {
                abilities.push(format!("+{} AGI", self.agi));
            }
            if self.wit > 0 {
                abilities.push(format!("+{} WIT", self.wit));
            }
            if !abilities.is_empty() {
                comps.push(format!("**Abilities**: {}.", abilities.join(", ")));
            }
        }

        {
            if !self.skills.is_empty() {
                comps.push(format!("**Skills**: {}.", self.skills.join(", ")));
            }
        }

        {
            let mut trappings = Vec::new();

            let item_map = count_map(&self.items);
            for (item, count) in item_map {
                if count > 1 {
                    trappings.push(format!("{}× {}", count, item.markdownify()));
                } else {
                    trappings.push(item.markdownify());
                }
            }

            for follower in self.followers.iter() {
                trappings.push(follower.markdownify())
            }

            if self.money > 0 {
                trappings.push(format!("{}ʂ", self.money));
            }

            if !trappings.is_empty() {
                comps.push(format!("**Trappings**: {}.", trappings.join(", ")));
            }
        }

        comps.join("\\\n")
    }
}

impl Markdownify for CharacterSize {
    fn markdownify(&self) -> String {
        format!("{:?}", self).to_case(Case::Lower)
    }
}

impl Markdownify for Character {
    fn markdownify(&self) -> String {
        let mut comps = Vec::new();

        // Name, cost
        {
            let cost_string = match self.cost {
                Some(cost) => format!(" ({}ʂ)", cost),
                None => String::new(),
            };
            comps.push(format!(
                "**{}**{}",
                capitalise(self.name.clone()),
                cost_string,
            ));
        }

        // Descr
        if !self.descr.is_empty() {
            comps.push(format!("*{}*", self.descr));
        }

        // Properties.
        {
            let mut properties = Vec::new();
            properties.push(format!(
                "**STR:** {}, **AGI:** {}, **WIT:** {}",
                self.str, self.agi, self.wit
            ));
            if self.traits.iter().find(|x| x.name == "tough").is_some() {
                properties.push(format!("**Health:** {}", self.str + 3));
            }
            if self.armour > 0 {
                properties.push(format!("**Armour:** {}", self.armour));
            }
            if self.omens > 0 {
                properties.push(format!("**Omens:** {}", self.omens));
            }
            if self.mana > 0 {
                properties.push(format!("**Mana:** {}", self.mana));
            }
            if self.size != CharacterSize::Medium {
                properties.push(format!("**Size:** {}", self.size.markdownify()));
            }
            let mut carry_limit = match self.size {
                CharacterSize::Medium => 16,
                CharacterSize::Tiny => 2,
                CharacterSize::Huge => 128,
            };
            if self.traits.iter().find(|x| x.name == "sturdy").is_some() {
                carry_limit *= 2;
            }
            if carry_limit != 16 {
                properties.push(format!("**Carry limit:** {}", carry_limit));
            }

            if !self.attacks.is_empty() {
                properties.push(format!(
                    "**Attacks:** {}",
                    self.attacks
                        .iter()
                        .map(|x| format!("{} ({})", x.0, x.1.markdownify()))
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
            }

            if !self.traits.is_empty() {
                properties.push(format!(
                    "**Traits:** {}",
                    self.traits
                        .iter()
                        .map(|x| x.markdownify())
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
            }

            {
                let money_str = if self.money > 0 {
                    format!(", {}ʂ", self.money)
                } else {
                    String::new()
                };
                if !self.items.is_empty() {
                    properties.push(format!(
                        "**Items:** {}{}",
                        self.items
                            .iter()
                            .map(|x| x.markdownify())
                            .collect::<Vec<String>>()
                            .join(", "),
                        money_str
                    ));
                }
            }
            comps.push(format!("{}.", properties.join(". ")));
        }

        // Special rules
        for (n, d) in self.special_traits.iter() {
            comps.push(format!("**{}.** {}", capitalise(n.to_string()), d));
        }

        comps.join("\\\n")
    }
}

fn gen_table(dir: &Path, title: &str, items: &[String], columns: usize) -> std::io::Result<()> {
    assert!(columns > 0);
    let tag = format!("tb_{}", title.to_case(Case::Snake));
    let mut file = File::create(dir.join(format!("{tag}.md")))?;

    let die_size = items.len() / columns;

    if columns > 1 {
        write!(&mut file, "|D{die_size}\\D{columns}|")?;
        for i in 1..=columns {
            write!(&mut file, "{i}|")?;
        }
        write!(&mut file, "\n")?;
        for _ in 1..=(columns + 1) {
            write!(&mut file, "|:-:")?;
        }
        write!(&mut file, "|\n")?;
    } else {
        write!(&mut file, "|D{die_size}||\n")?;
        write!(&mut file, "|:-:|---|\n")?;
    }

    for (i, x) in items.iter().enumerate() {
        if i % columns == 0 {
            write!(&mut file, "|{}", i / columns + 1)?;
        }
        write!(&mut file, "|{}", capitalise(x.clone()))?;
        if i % columns == (columns - 1) {
            write!(&mut file, "|\n")?;
        }
    }
    Ok(())
}

fn find_item(name: &str, items: &BTreeMap<String, Vec<Item>>) -> Option<Item> {
    for (_, items) in items.iter() {
        if let Some(item) = items.iter().find(|x| x.name == name) {
            return Some(item.clone());
        }
    }
    None
}

fn find_character(name: &str, characters: &BTreeMap<String, Vec<Character>>) -> Option<Character> {
    for (_, characters) in characters.iter() {
        if let Some(character) = characters.iter().find(|x| x.name == name) {
            return Some(character.clone());
        }
    }
    None
}

fn fill_career_money(
    careers: &mut [Career],
    items: &BTreeMap<String, Vec<Item>>,
    characters: &BTreeMap<String, Vec<Character>>,
) {
    for career in careers.iter_mut() {
        let mut money = 44;
        for item in career.items.iter() {
            let item_profile = find_item(&item.name, items).expect("Item not found.");
            let mut item_value = item_profile.value;
            if item.descr.contains("expensive") {
                item_value *= 4;
            }
            if item.descr.contains("fragile") {
                item_value /= 4;
            }
            if item.descr.contains("stolen") {
                item_value /= 4;
            }
            money -= item_value;
        }
        for follower in career.followers.iter() {
            let follower_profile =
                find_character(&follower.name, characters).expect("Follower not found.");
            let mut follower_value = follower_profile.cost.unwrap_or(0);
            if follower.descr.contains("tamed") {
                follower_value *= 2;
            }
            money -= follower_value;
        }
        money = money / 2;
        if money < 0 {
            println!(
                "WARNING: negative money ({}) for {}",
                money, career.male_name
            );
        }
        career.money = 0.max(money);
    }
}

fn generate_rules() -> Result<(), Box<dyn Error>> {
    let base_path = Path::new("../gen");

    let items: BTreeMap<String, Vec<Item>> =
        serde_yaml::from_reader(File::open("../game_data/items.yml")?)?;
    let relics: Vec<Item> = serde_yaml::from_reader(File::open("../game_data/relics.yml")?)?;
    let spells: BTreeMap<String, Vec<Spell>> =
        serde_yaml::from_reader(File::open("../game_data/spells.yml")?)?;
    let description: BTreeMap<String, Vec<String>> =
        serde_yaml::from_reader(File::open("../game_data/description.yml")?)?;
    let skills: BTreeMap<String, String> =
        serde_yaml::from_reader(File::open("../game_data/skills.yml")?)?;
    let traits: BTreeMap<String, String> =
        serde_yaml::from_reader(File::open("../game_data/traits.yml")?)?;
    let characters: BTreeMap<String, Vec<Character>> =
        serde_yaml::from_reader(File::open("../game_data/bestiary.yml")?)?;
    let mut careers: Vec<Career> =
        serde_yaml::from_reader(File::open("../game_data/careers.yml")?)?;
    let extra_items: Vec<AssetWithDescr> =
        serde_yaml::from_reader(File::open("../game_data/extra_items.yml")?)?;

    fill_career_money(&mut careers, &items, &characters);

    for (category, items) in items.iter() {
        let filename = format!("ref_{}.md", category.to_case(Case::Snake));
        let mut f = File::create(base_path.join(filename))?;
        for item in items {
            writeln!(f, "* {}", item.markdownify())?;
        }
    }

    {
        let mut f = File::create(base_path.join("ref_relics.md"))?;
        for relic in relics {
            writeln!(f, "1. {}", relic.markdownify())?;
        }
    }

    for (category, spells) in spells.iter() {
        let label = category.to_case(Case::Snake);

        let filename = format!("ref_{label}.md");
        let mut f = File::create(base_path.join(filename))?;
        for spell in spells {
            writeln!(f, "* {}", spell.markdownify())?;
        }

        let spell_names = spells
            .iter()
            .map(|x| x.name.clone())
            .collect::<Vec<String>>();
        gen_table(&base_path, &label, &spell_names, 2)?;
    }

    for (category, characters) in characters {
        let filename = format!("ref_{}.md", category.to_case(Case::Snake));
        let mut f = File::create(base_path.join(filename))?;
        for character in characters {
            writeln!(f, "{}\n", character.markdownify())?;
        }
    }

    for (category, entries) in description.iter() {
        let columns = if entries.len() == 20 || entries.len() == 40 {
            2
        } else {
            4
        };
        gen_table(&base_path, &capitalise(category.clone()), &entries, columns)?;
    }

    {
        let mut f = File::create(base_path.join("ref_skills.md"))?;
        for (name, descr) in skills.iter() {
            writeln!(f, "* **{}**.\n{}", capitalise(name.clone()), descr)?;
        }

        let mut starting_skills = Vec::new();
        for (name, _) in skills.iter() {
            if name != "literate" {
                starting_skills.push(name.clone());
            }
        }
        while starting_skills.len() < 80 {
            starting_skills.push("literate".to_string());
        }
        gen_table(&base_path, "Starting skills", &starting_skills, 4)?;
    }

    {
        let mut f = File::create(base_path.join("ref_traits.md"))?;
        for (name, descr) in traits.iter() {
            writeln!(f, "* **{}**.\n{}", capitalise(name.clone()), descr)?;
        }
    }

    {
        let career_names = careers
            .iter()
            .map(|x| x.male_name.clone())
            .collect::<Vec<String>>();

        let mut f = File::create(base_path.join("ref_careers.md"))?;
        for career in careers {
            writeln!(
                f,
                "## {}\n\n{}\n",
                capitalise(career.male_name.clone()),
                career.markdownify()
            )?;
        }

        gen_table(&base_path, "Careers", &career_names, 4)?;
    }

    {
        let items = extra_items
            .iter()
            .map(|x| x.markdownify())
            .collect::<Vec<String>>();
        gen_table(&base_path, "Extra items", &items, 4)?;
    }

    Ok(())
}

fn process_str_for_js(mut s: String) -> String {
    let re = Regex::new(r"\*(?<s>[^\*]*)\*").unwrap();
    re.replace_all(&mut s, r"<i>${s}</i>").to_string()
}

pub fn generate_javascript() -> Result<(), Box<dyn Error>> {
    let output_dir = Path::new("../assets/js");
    let mut f = File::create(output_dir.join("data.js"))?;

    {
        let items: BTreeMap<String, Vec<Item>> =
            serde_yaml::from_reader(File::open("../game_data/items.yml")?)?;
        let characters: BTreeMap<String, Vec<Character>> =
            serde_yaml::from_reader(File::open("../game_data/bestiary.yml")?)?;
        let mut careers: Vec<Career> =
            serde_yaml::from_reader(File::open("../game_data/careers.yml")?)?;
        fill_career_money(&mut careers, &items, &characters);

        writeln!(f, "export const careers = [\n")?;
        for career in careers {
            writeln!(f, "  {{")?;
            writeln!(
                f,
                "    male_name: \"{}\",",
                career.male_name.to_case(Case::Title)
            )?;
            writeln!(
                f,
                "    female_name: \"{}\",",
                career.female_name.to_case(Case::Title)
            )?;
            writeln!(
                f,
                "    description: \"{}\",",
                process_str_for_js(career.descr)
            )?;
            writeln!(f, "    str: {},", career.str)?;
            writeln!(f, "    agi: {},", career.agi)?;
            writeln!(f, "    wit: {},", career.wit)?;
            writeln!(f, "    skills: [")?;
            for skill in career.skills.iter() {
                writeln!(f, "      \"{}\",", skill)?;
            }
            writeln!(f, "    ],")?;
            writeln!(f, "    trappings: [")?;
            for item in career.items {
                writeln!(f, "      \"{}\",", process_str_for_js(item.markdownify()))?;
            }
            for follower in career.followers {
                writeln!(
                    f,
                    "      \"{}\",",
                    process_str_for_js(follower.markdownify())
                )?;
            }
            if career.money > 0 {
                writeln!(f, "      \"{}ʂ\",", career.money)?;
            }
            writeln!(f, "    ],")?;
            writeln!(f, "  }},")?;
        }
        writeln!(f, "];\n")?;
    }

    {
        let skills: BTreeMap<String, String> =
            serde_yaml::from_reader(File::open("../game_data/skills.yml")?)?;

        writeln!(f, "export const skill_descriptions = {{")?;
        for (skill_name, skill_descr) in skills.iter() {
            writeln!(
                f,
                "  \"{}\": \"{}\",",
                skill_name,
                process_str_for_js(skill_descr.clone())
            )?;
        }
        writeln!(f, "}};\n")?;

        let mut starting_skills = Vec::new();
        for (name, _) in skills.iter() {
            if name != "literate" {
                starting_skills.push(name.clone());
            }
        }
        while starting_skills.len() < 80 {
            starting_skills.push("literate".to_string());
        }

        writeln!(f, "export const starting_skills = [")?;
        for skill in starting_skills {
            writeln!(f, "  \"{}\",", skill)?;
        }
        writeln!(f, "];\n")?;
    }

    {
        let extra_items: Vec<AssetWithDescr> =
            serde_yaml::from_reader(File::open("../game_data/extra_items.yml")?)?;

        writeln!(f, "export const extra_items = [")?;
        for item in extra_items.iter() {
            let mut item = item.clone();
            if item.name.contains("poison") {
                item.descr.clear();
            }
            writeln!(f, "  \"{}\",", process_str_for_js(item.markdownify()))?;
        }
        writeln!(f, "];\n")?;
    }

    {
        let spells: BTreeMap<String, Vec<Spell>> =
            serde_yaml::from_reader(File::open("../game_data/spells.yml")?)?;
        for (category, entries) in spells {
            let spell_names = entries
                .iter()
                .map(|x| x.name.clone())
                .collect::<Vec<String>>();
            writeln!(f, "export const {} = [", category.to_case(Case::Snake))?;
            for v in spell_names.iter() {
                writeln!(f, "  \"{v}\",")?;
            }
            writeln!(f, "];\n")?;
        }
    }

    {
        writeln!(f, "export const genders = [")?;
        for v in ["Male", "Female"] {
            writeln!(f, "  \"{v}\",")?;
        }
        writeln!(f, "];\n")?;
    }

    {
        let description: BTreeMap<String, Vec<String>> =
            serde_yaml::from_reader(File::open("../game_data/description.yml")?)?;
        for (category, entries) in description {
            writeln!(f, "export const {} = [", category.to_case(Case::Snake))?;
            for v in entries.iter() {
                writeln!(f, "  \"{}\",", process_str_for_js(v.clone()))?;
            }
            writeln!(f, "];\n")?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    generate_rules()?;
    generate_javascript()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_items() -> Vec<String> {
        let mut ans = Vec::new();
        let items: BTreeMap<String, Vec<Item>> =
            serde_yaml::from_reader(File::open("../game_data/items.yml").unwrap()).unwrap();
        for (_, items) in items {
            for item in items {
                ans.push(item.name);
            }
        }
        ans
    }

    fn all_traits() -> Vec<String> {
        let mut ans = Vec::new();
        let skills: BTreeMap<String, String> =
            serde_yaml::from_reader(File::open("../game_data/skills.yml").unwrap()).unwrap();
        for (skill, _) in skills {
            ans.push(skill);
        }
        let traits: BTreeMap<String, String> =
            serde_yaml::from_reader(File::open("../game_data/traits.yml").unwrap()).unwrap();
        for (t, _) in traits {
            ans.push(t);
        }
        ans
    }

    fn all_characters() -> Vec<String> {
        let mut ans = Vec::new();
        let characters: BTreeMap<String, Vec<Character>> =
            serde_yaml::from_reader(File::open("../game_data/bestiary.yml").unwrap()).unwrap();
        for (_, characters) in characters {
            for character in characters {
                ans.push(character.name);
            }
        }
        ans
    }

    #[test]
    fn skills() {
        let _items: BTreeMap<String, String> =
            serde_yaml::from_reader(File::open("../game_data/skills.yml").unwrap()).unwrap();
    }

    #[test]
    fn traits() {
        let _items: BTreeMap<String, String> =
            serde_yaml::from_reader(File::open("../game_data/traits.yml").unwrap()).unwrap();
    }

    #[test]
    fn items() {
        let _items: BTreeMap<String, Vec<Item>> =
            serde_yaml::from_reader(File::open("../game_data/items.yml").unwrap()).unwrap();
    }

    #[test]
    fn relics() {
        let _relics: Vec<Item> =
            serde_yaml::from_reader(File::open("../game_data/relics.yml").unwrap()).unwrap();
    }

    #[test]
    fn spells() {
        let _items: BTreeMap<String, Vec<Spell>> =
            serde_yaml::from_reader(File::open("../game_data/spells.yml").unwrap()).unwrap();
    }

    #[test]
    fn bestiary() {
        let characters: BTreeMap<String, Vec<Character>> =
            serde_yaml::from_reader(File::open("../game_data/bestiary.yml").unwrap()).unwrap();
        let trts = all_traits();
        let items = all_items();
        for (_, characters) in characters {
            for character in characters {
                for trat in character.traits {
                    assert!(trts.contains(&trat.name), "Invalid trait {}", trat.name);
                }
                for item in character.items {
                    assert!(items.contains(&item.name), "Invalid item {}", item.name);
                }
            }
        }
    }

    #[test]
    fn description() {
        let _items: BTreeMap<String, Vec<String>> =
            serde_yaml::from_reader(File::open("../game_data/description.yml").unwrap()).unwrap();
    }

    #[test]
    fn careers() {
        let careers: Vec<Career> =
            serde_yaml::from_reader(File::open("../game_data/careers.yml").unwrap()).unwrap();
        let skills = all_traits();
        let items = all_items();
        let characters = all_characters();
        for career in careers {
            for skill in career.skills {
                assert!(skills.contains(&skill), "Invalid skill {}", skill);
            }
            for item in career.items {
                assert!(items.contains(&item.name), "Invalid item {}", item.name);
            }
            for follower in career.followers {
                assert!(
                    characters.contains(&follower.name),
                    "Invalid follower {}",
                    follower.name
                );
            }
        }
    }
}
