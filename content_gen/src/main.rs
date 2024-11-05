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

fn count_map(assets: &[AssetWithDescr]) -> BTreeMap<AssetWithDescr, i32> {
    let mut ans = BTreeMap::new();
    for asset in assets.iter() {
        *ans.entry(asset.clone()).or_default() += 1;
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
    DirectDamage,
    FireDamage,
    Grab,
    HolyDamage,
    IndirectRange(i8),
    Poison(String),
    Range(i8),
    Reload,
    Stun,
    UsageLimit(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Weapon {
    #[serde(default)]
    damage: Option<Die>,
    #[serde(default)]
    properties: Vec<AttackProperty>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum ItemProperty {
    Armour(i8),
    Consumable,
    Health(i8, i8),
    Fragile,
    Robust,
    Shield,
    Vehicle(i8),
    Weapon(Weapon),
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Item {
    name: String,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum PowerRange {
    Personal,
    Near,
    Sight,
    Connection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum PowerDuration {
    Instant,
    Stretch,
    Watch,
    Lingering,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Power {
    name: String,
    descr: String,
    range: PowerRange,
    duration: PowerDuration,
    #[serde(default)]
    enhancements: Vec<(i8, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Background {
    male_name: String,
    female_name: String,
    skills: Vec<String>,
    #[serde(default)]
    items: Vec<AssetWithDescr>,
    #[serde(default)]
    followers: Vec<AssetWithDescr>,
    #[serde(default)]
    money: i32,
    #[serde(default)]
    mana: i8,
    descr: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CharacterKind {
    Construct,
    Creature,
    Demon,
    Undead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CharacterSize {
    Tiny,
    Small,
    Medium,
    Large,
    Massive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CharacterIntelligence {
    HumanIntelligence,
    AnimalIntelligence,
    Mindless,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Character {
    name: String,
    cost: Option<i32>,
    descr: String,
    kind: CharacterKind,
    size: CharacterSize,
    intelligence: CharacterIntelligence,
    str: i8,
    agi: i8,
    wit: i8,
    mana: i8,
    skills: Vec<String>,
    traits: Vec<String>,
    special_traits: BTreeMap<String, String>,
    natural_armour: Option<(String, i8)>,
    natural_weapons: Vec<(String, Weapon)>,
    items: Vec<AssetWithDescr>,
    money: i32,
}

trait ToAsciiDoc {
    fn to_asciidoc(&self) -> String;
}

impl ToAsciiDoc for Die {
    fn to_asciidoc(&self) -> String {
        match self {
            Self::D4 => "d4".to_string(),
            Self::D6 => "d6".to_string(),
            Self::D8 => "d8".to_string(),
            Self::D10 => "d10".to_string(),
            Self::D12 => "d12".to_string(),
        }
    }
}

impl ToAsciiDoc for AttackProperty {
    fn to_asciidoc(&self) -> String {
        match self {
            Self::Range(x) => format!("range {x}"),
            Self::IndirectRange(x) => format!("range {x}"),
            Self::Poison(x) => format!("poison ({x})"),
            Self::UsageLimit(x) => format!("usage limit ({x})"),
            _ => format!("{:?}", self).to_case(Case::Lower),
        }
    }
}

impl ToAsciiDoc for Weapon {
    fn to_asciidoc(&self) -> String {
        let mut comps = Vec::new();
        if let Some(die) = self.damage {
            comps.push(die.to_asciidoc());
        }
        for property in self.properties.iter() {
            comps.push(property.to_asciidoc())
        }
        comps.join(", ")
    }
}

impl ToAsciiDoc for ItemProperty {
    fn to_asciidoc(&self) -> String {
        match self {
            Self::Armour(x) => format!("armour {x}"),
            Self::Health(x, y) => format!("health {x}/{y}"),
            Self::Vehicle(x) => format!("vehicle {x}"),
            Self::Weapon(x) => format!("weapon ({})", x.to_asciidoc()),
            _ => format!("{:?}", self).to_case(Case::Lower),
        }
    }
}

impl ToAsciiDoc for Item {
    fn to_asciidoc(&self) -> String {
        let mut comps = Vec::new();

        let name = capitalise(self.name.clone());
        let bulk = match self.bulk {
            0 => ", bulk ½".to_string(),
            1 => String::new(),
            x => format!(", bulk {x}"),
        };
        let cost_and_bulk = format!("{}ʂ{}", self.value, bulk);
        comps.push(format!("*{name}* ({cost_and_bulk})"));

        if !self.properties.is_empty() {
            let properties = capitalise(
                self.properties
                    .iter()
                    .map(|x| format!("_{}_", x.to_asciidoc()))
                    .collect::<Vec<String>>()
                    .join(", "),
            );
            comps.push(properties);
        }

        if !self.descr.is_empty() {
            let descr = capitalise(self.descr.clone());
            comps.push(descr);
        }

        comps.join(".\n")
    }
}

impl ToAsciiDoc for AssetWithDescr {
    fn to_asciidoc(&self) -> String {
        let mut ans = self.name.clone();
        if !self.descr.is_empty() {
            ans += &format!(" ({})", self.descr);
        }
        ans
    }
}

impl ToAsciiDoc for PowerDuration {
    fn to_asciidoc(&self) -> String {
        format!("{:?}", self).to_case(Case::Lower)
    }
}

impl ToAsciiDoc for PowerRange {
    fn to_asciidoc(&self) -> String {
        format!("{:?}", self).to_case(Case::Lower)
    }
}

impl ToAsciiDoc for Power {
    fn to_asciidoc(&self) -> String {
        let mut comps = Vec::new();
        comps.push(format!("*{}*.", capitalise(self.name.clone())));
        comps.push(format!(
            "_{}_, _{}_.",
            capitalise(self.range.to_asciidoc()),
            self.duration.to_asciidoc()
        ));
        comps.push(self.descr.clone());
        for enhancement in self.enhancements.iter() {
            comps.push(format!("* {} EP -- {}", enhancement.0, enhancement.1));
        }
        comps.join("\n")
    }
}

impl ToAsciiDoc for Background {
    fn to_asciidoc(&self) -> String {
        let mut comps = Vec::new();

        comps.push(format!("{}\n", self.descr));

        comps.push(format!(
            "*Skills*: {}.\n",
            self.skills
                .iter()
                .map(|x| format!("_{x}_"))
                .collect::<Vec<String>>()
                .join(", ")
        ));

        let money_str = if self.money > 0 {
            format!("{}ʂ", self.money)
        } else {
            String::new()
        };

        let item_map = count_map(&self.items);
        comps.push(format!(
            "*Items*: {}{}.\n",
            item_map
                .iter()
                .map(|(x, c)| {
                    if *c > 1 {
                        format!("{}× {}", c, x.to_asciidoc())
                    } else {
                        x.to_asciidoc()
                    }
                })
                .collect::<Vec<String>>()
                .join(", "),
            money_str
        ));

        if !self.followers.is_empty() {
            comps.push(format!(
                "*Followers*: {}.\n",
                self.followers
                    .iter()
                    .map(|x| x.to_asciidoc())
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }

        comps.join("\n")
    }
}

impl ToAsciiDoc for CharacterKind {
    fn to_asciidoc(&self) -> String {
        format!("{:?}", self).to_case(Case::Lower)
    }
}

impl ToAsciiDoc for CharacterSize {
    fn to_asciidoc(&self) -> String {
        format!("{:?}", self).to_case(Case::Lower)
    }
}

impl ToAsciiDoc for CharacterIntelligence {
    fn to_asciidoc(&self) -> String {
        format!("{:?}", self).to_case(Case::Lower)
    }
}

impl ToAsciiDoc for Character {
    fn to_asciidoc(&self) -> String {
        let mut comps = Vec::new();

        let cost_string = match self.cost {
            Some(cost) => format!(" ({}ʂ)", cost),
            None => String::new(),
        };
        comps.push(format!(
            "*{}*{}",
            capitalise(self.name.clone()),
            cost_string
        ));
        if !self.descr.is_empty() {
            comps.push(self.descr.clone());
        }

        let mana_str = if self.mana > 0 {
            format!(", Mana {}", self.mana)
        } else {
            String::new()
        };
        comps.push(format!(
            "*STR* {}, *AGI* {}, *WIT* {}{}",
            self.str, self.agi, self.wit, mana_str,
        ));

        comps.push(format!(
            "_{} {}_, {}.",
            capitalise(self.size.to_asciidoc()),
            self.kind.to_asciidoc(),
            self.intelligence.to_asciidoc()
        ));

        if !self.skills.is_empty() {
            comps.push(format!(
                "*Skills*: {}.",
                self.skills
                    .iter()
                    .map(|x| format!("_{x}_"))
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }

        if !self.traits.is_empty() {
            comps.push(format!(
                "*Traits*: {}.",
                self.traits
                    .iter()
                    .map(|x| format!("_{x}_"))
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }

        for (n, d) in self.special_traits.iter() {
            comps.push(format!("*{}*. {}", capitalise(n.to_string()), d));
        }

        if let Some((n, av)) = self.natural_armour.clone() {
            comps.push(format!("*{}*. Armor value {av}.", capitalise(n)))
        }

        for (nat_weapon, stats) in self.natural_weapons.iter() {
            comps.push(format!(
                "*{}*. {}.",
                capitalise(nat_weapon.clone()),
                stats.to_asciidoc()
            ))
        }

        let money_str = if self.money > 0 {
            format!(", {}ʂ", self.money)
        } else {
            String::new()
        };
        if !self.items.is_empty() {
            comps.push(format!(
                "*Items*: {}{}.",
                self.items
                    .iter()
                    .map(|x| x.to_asciidoc())
                    .collect::<Vec<String>>()
                    .join(", "),
                money_str
            ));
        }

        comps.join(" +\n")
    }
}

fn gen_double_die_table(
    dir: &Path,
    title: &str,
    items: &[String],
    columns: usize,
) -> std::io::Result<()> {
    assert!(columns > 1);
    let tag = format!("tb_{}", title.to_case(Case::Snake));
    let mut file = File::create(dir.join(format!("{tag}.adoc")))?;

    let die_size = items.len() / columns;
    let column_size = 14 / columns;

    writeln!(&mut file, ".{title}")?;
    writeln!(&mut file, "[[{tag}]]")?;
    writeln!(
        &mut file,
        r#"[options='header, unbreakable', cols="^1h,{}"]"#,
        vec![format!("^{column_size}"); columns].join(",")
    )?;
    writeln!(&mut file, "|===")?;

    writeln!(&mut file, "h|  {columns}+h|D{columns}")?;
    writeln!(&mut file, "h|D{die_size}")?;
    for i in 1..=columns {
        write!(&mut file, " h|{i}")?;
    }
    write!(&mut file, "\n")?;

    for (i, x) in items.iter().enumerate() {
        if i % columns == 0 {
            writeln!(&mut file, "|{}", i / columns + 1)?;
        }
        writeln!(&mut file, "|{}", capitalise(x.clone()))?;
    }
    writeln!(&mut file, "|===")?;
    Ok(())
}

fn generate_rules() -> Result<(), Box<dyn Error>> {
    let base_path = Path::new("../gen");

    {
        let items: BTreeMap<String, Vec<Item>> =
            serde_yaml::from_reader(File::open("../game_data/items.yml")?)?;
        for (category, items) in items {
            let filename = format!("ref_{}.adoc", category.to_case(Case::Snake));
            let mut f = File::create(base_path.join(filename))?;
            let header = capitalise(category);
            writeln!(f, "== {header}\n")?;
            for item in items {
                writeln!(f, "* {}\n", item.to_asciidoc())?;
            }
        }
    }

    {
        let characters: BTreeMap<String, Vec<Character>> =
            serde_yaml::from_reader(File::open("../game_data/characters.yml")?)?;
        for (category, characters) in characters {
            let filename = format!("ref_{}.adoc", category.to_case(Case::Snake));
            let mut f = File::create(base_path.join(filename))?;
            let header = capitalise(category);
            writeln!(f, "== {header}\n")?;
            for character in characters {
                writeln!(f, "* {}\n", character.to_asciidoc())?;
            }
        }
    }

    {
        let backgrounds: Vec<Background> =
            serde_yaml::from_reader(File::open("../game_data/backgrounds.yml")?)?;

        let background_names = backgrounds
            .iter()
            .map(|x| x.male_name.clone())
            .collect::<Vec<String>>();

        let mut f = File::create(base_path.join("ref_backgrounds.adoc"))?;
        for bg in backgrounds {
            writeln!(
                f,
                "== {}\n\n{}\n",
                capitalise(bg.male_name.clone()),
                bg.to_asciidoc()
            )?;
        }

        gen_double_die_table(&base_path, "backgrounds", &background_names, 4)?;
    }

    {
        let powers: BTreeMap<String, Vec<Power>> =
            serde_yaml::from_reader(File::open("../game_data/powers.yml")?)?;

        for (category, powers) in powers {
            let header = capitalise(category.clone());

            let power_names = powers
                .iter()
                .map(|x| x.name.clone())
                .collect::<Vec<String>>();

            let filename = format!("ref_{}.adoc", category.to_case(Case::Snake));
            let mut f = File::create(base_path.join(filename))?;
            writeln!(f, "== {header}\n")?;
            for power in powers {
                writeln!(f, "* {}\n", power.to_asciidoc())?;
            }

            gen_double_die_table(&base_path, &header, &power_names, 2)?;
        }
    }

    {
        let skills: BTreeMap<String, String> =
            serde_yaml::from_reader(File::open("../game_data/skills.yml")?)?;
        let mut f = File::create(base_path.join("ref_skills.adoc"))?;
        writeln!(f, "== Skill list\n")?;
        for (name, descr) in skills {
            writeln!(f, "* *{}*.\n{}\n", capitalise(name), descr)?;
        }
    }

    {
        let traits: BTreeMap<String, String> =
            serde_yaml::from_reader(File::open("../game_data/traits.yml")?)?;
        let mut f = File::create(base_path.join("ref_traits.adoc"))?;
        writeln!(f, "== Trait list\n")?;
        for (name, descr) in traits {
            writeln!(f, "* *{}*.\n{}\n", capitalise(name), descr)?;
        }
    }

    {
        let conditions: BTreeMap<String, String> =
            serde_yaml::from_reader(File::open("../game_data/conditions.yml")?)?;
        let mut f = File::create(base_path.join("ref_conditions.adoc"))?;
        writeln!(f, "== Condition list\n")?;
        for (name, descr) in conditions {
            writeln!(f, "* *{}*.\n{}\n", capitalise(name), descr)?;
        }
    }

    {
        let description: BTreeMap<String, Vec<String>> =
            serde_yaml::from_reader(File::open("../game_data/description.yml")?)?;
        for (category, entries) in description {
            gen_double_die_table(&base_path, &capitalise(category), &entries, 2)?;
        }
    }

    {
        let starting_assets: BTreeMap<String, Vec<(AssetWithDescr, i32)>> =
            serde_yaml::from_reader(File::open("../game_data/starting_assets.yml")?)?;
        for (category, entries) in starting_assets {
            let entries = entries
                .iter()
                .map(|x| {
                    let mut comps = Vec::new();
                    comps.push(x.0.to_asciidoc());
                    if x.1 > 0 {
                        comps.push(format!("{}ʂ", x.1));
                    }
                    comps.join(", ")
                })
                .collect::<Vec<String>>();
            gen_double_die_table(&base_path, &capitalise(category), &entries, 2)?;
        }
    }

    Ok(())
}

fn process_keywords_for_js(mut s: String) -> String {
    let re = Regex::new(r"_(?<s>[^_]*)_").unwrap();
    re.replace_all(&mut s, r"<i>${s}</i>")
        .to_string()
        .replace("'`", "'")
        .replace("`'", "'")
}

pub fn generate_javascript() -> Result<(), Box<dyn Error>> {
    let output_dir = Path::new("../assets/js");
    let mut f = File::create(output_dir.join("data.js"))?;

    {
        let backgrounds: Vec<Background> =
            serde_yaml::from_reader(File::open("../game_data/backgrounds.yml")?)?;

        writeln!(f, "export const backgrounds = [")?;
        for background in backgrounds {
            let mana = background.mana;
            let skills = background.skills;
            let money = background.money;

            let mut items = Vec::new();
            let mut followers = Vec::new();
            let mut profane_scrolls = 0;
            let mut sacred_scrolls = 0;

            for item in background.items {
                if item.name == "power scroll" {
                    if item.descr == "profane" {
                        profane_scrolls += 1;
                    } else if item.descr == "sacred" {
                        sacred_scrolls += 1;
                    } else {
                        panic!("Unsupported scroll type");
                    }
                } else {
                    items.push(item.to_asciidoc())
                }
            }

            for follower in background.followers {
                followers.push(follower.to_asciidoc())
            }

            writeln!(f, "  {{")?;
            writeln!(
                f,
                "    masculine_name: \"{}\",",
                background.male_name.to_case(Case::Title)
            )?;
            writeln!(
                f,
                "    feminine_name: \"{}\",",
                background.female_name.to_case(Case::Title)
            )?;
            writeln!(
                f,
                "    description: \"{}\",",
                process_keywords_for_js(background.descr)
            )?;

            writeln!(f, "    skills: [")?;
            for skill in skills.iter() {
                writeln!(f, "      \"{}\",", skill)?;
            }
            writeln!(f, "    ],")?;

            writeln!(f, "    mana: {},", mana)?;

            writeln!(f, "    items: [")?;
            for item in items {
                writeln!(f, "      \"{}\",", process_keywords_for_js(item))?;
            }
            writeln!(f, "    ],")?;

            writeln!(f, "    followers: [")?;
            for follower in followers {
                writeln!(f, "      \"{}\",", process_keywords_for_js(follower))?;
            }
            writeln!(f, "    ],")?;

            writeln!(f, "    sacred_scrolls: {},", sacred_scrolls)?;
            writeln!(f, "    profane_scrolls: {},", profane_scrolls)?;
            writeln!(f, "    money: {},", money)?;

            writeln!(f, "  }},")?;
        }
        writeln!(f, "];\n")?;
    }

    {
        let assets: BTreeMap<String, Vec<(AssetWithDescr, i32)>> =
            serde_yaml::from_reader(File::open("../game_data/starting_assets.yml")?)?;
        for (category, entries) in assets {
            writeln!(f, "export const {} = [", category.to_case(Case::Snake))?;
            for (item, money) in entries {
                writeln!(
                    f,
                    "  [\"{}\", {}],",
                    process_keywords_for_js(item.to_asciidoc()),
                    money
                )?;
            }
            writeln!(f, "];\n")?;
        }
    }

    {
        let powers: BTreeMap<String, Vec<Power>> =
            serde_yaml::from_reader(File::open("../game_data/powers.yml")?)?;
        for (category, entries) in powers {
            let power_names = entries
                .iter()
                .map(|x| x.name.clone())
                .collect::<Vec<String>>();
            writeln!(f, "export const {} = [", category.to_case(Case::Snake))?;
            for v in power_names.iter() {
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
                writeln!(f, "  \"{}\",", process_keywords_for_js(v.clone()))?;
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

    fn all_skills() -> Vec<String> {
        let mut ans = Vec::new();
        let skills: BTreeMap<String, String> =
            serde_yaml::from_reader(File::open("../game_data/skills.yml").unwrap()).unwrap();
        for (skill, _) in skills {
            ans.push(skill);
        }
        ans
    }

    fn all_traits() -> Vec<String> {
        let mut ans = Vec::new();
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
            serde_yaml::from_reader(File::open("../game_data/characters.yml").unwrap()).unwrap();
        for (_, characters) in characters {
            for character in characters {
                ans.push(character.name);
            }
        }
        ans
    }

    #[test]
    fn items() {
        let _items: BTreeMap<String, Vec<Item>> =
            serde_yaml::from_reader(File::open("../game_data/items.yml").unwrap()).unwrap();
    }

    #[test]
    fn powers() {
        let _items: BTreeMap<String, Vec<Power>> =
            serde_yaml::from_reader(File::open("../game_data/powers.yml").unwrap()).unwrap();
    }

    #[test]
    fn backgrounds() {
        let backgrounds: Vec<Background> =
            serde_yaml::from_reader(File::open("../game_data/backgrounds.yml").unwrap()).unwrap();
        let skills = all_skills();
        let items = all_items();
        let characters = all_characters();
        for background in backgrounds {
            for skill in background.skills {
                assert!(skills.contains(&skill), "Invalid skill {}", skill);
            }
            for item in background.items {
                assert!(items.contains(&item.name), "Invalid item {}", item.name);
            }
            for follower in background.followers {
                assert!(
                    characters.contains(&follower.name),
                    "Invalid follower {}",
                    follower.name
                );
            }
        }
    }

    #[test]
    fn characters() {
        let characters: BTreeMap<String, Vec<Character>> =
            serde_yaml::from_reader(File::open("../game_data/characters.yml").unwrap()).unwrap();
        let skills = all_skills();
        let traits = all_traits();
        let items = all_items();
        for (_, characters) in characters {
            for character in characters {
                for skill in character.skills {
                    assert!(skills.contains(&skill), "Invalid skill {}", skill);
                }
                for t in character.traits {
                    assert!(traits.contains(&t), "Invalid trait {}", t);
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
    fn starting_assets() {
        let _items: BTreeMap<String, Vec<(AssetWithDescr, i32)>> =
            serde_yaml::from_reader(File::open("../game_data/starting_assets.yml").unwrap())
                .unwrap();
    }
}
