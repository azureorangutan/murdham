use convert_case::{Case, Casing};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    error::Error,
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

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
    DirectDamage,
    FireDamage,
    Grab,
    HolyDamage,
    Poison(String),
    Range(i8),
    RangeWithMin(i8, i8),
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
    Touch,
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
struct Career {
    male_name: String,
    female_name: String,
    skills: Vec<String>,
    #[serde(default)]
    items: Vec<AssetWithDescr>,
    #[serde(default)]
    followers: Vec<AssetWithDescr>,
    money: i32,
    #[serde(default)]
    mana: i8,
    descr: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CharacterSize {
    Tiny,
    Small,
    Medium,
    Large,
    Massive,
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
            format!("_{}_", self.name)
        } else {
            format!("_{} ({})_", self.name, self.descr)
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
    size: CharacterSize,
    str: i8,
    agi: i8,
    wit: i8,
    #[serde(default)]
    mana: i8,
    #[serde(default)]
    omens: i8,
    #[serde(default)]
    skills: Vec<String>,
    #[serde(default)]
    traits: Vec<Trait>,
    #[serde(default)]
    special_traits: BTreeMap<String, String>,
    #[serde(default)]
    natural_armour: Option<(String, i8)>,
    #[serde(default)]
    natural_weapons: Vec<(String, Weapon)>,
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
            Self::Range(x) => format!("range {x}"),
            Self::RangeWithMin(x, y) => format!("range {x}--{y}"),
            Self::Poison(x) => format!("poison ({x})"),
            Self::UsageLimit(x) => format!("usage limit: {x}"),
            _ => format!("{:?}", self).to_case(Case::Lower),
        }
    }
}

impl Markdownify for Weapon {
    fn markdownify(&self) -> String {
        let mut comps = Vec::new();
        if let Some(die) = self.damage {
            comps.push(die.markdownify());
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
            Self::Armour(x) => format!("armour {x}"),
            Self::Health(x, y) => format!("health {x}/{y}"),
            Self::Vehicle(x) => format!("vehicle {x}"),
            Self::Weapon(x) => format!("weapon ({})", x.markdownify()),
            _ => format!("{:?}", self).to_case(Case::Lower),
        }
    }
}

impl Markdownify for Item {
    fn markdownify(&self) -> String {
        let mut comps = Vec::new();

        let name = capitalise(self.name.clone());
        let bulk = match self.bulk {
            0 => ", bulk ½".to_string(),
            1 => String::new(),
            x => format!(", bulk {x}"),
        };
        let cost_and_bulk = format!("{}ʂ{}", self.value, bulk);
        comps.push(format!("**{name}** ({cost_and_bulk})."));

        if !self.properties.is_empty() {
            let properties = capitalise(
                self.properties
                    .iter()
                    .map(|x| format!("_{}_", x.markdownify()))
                    .collect::<Vec<String>>()
                    .join(", "),
            );
            comps.push(format!("{properties}."));
        }

        if !self.descr.is_empty() {
            let descr = capitalise(self.descr.clone());
            comps.push(descr);
        }

        comps.join("\n")
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

impl Markdownify for PowerDuration {
    fn markdownify(&self) -> String {
        format!("{:?}", self).to_case(Case::Lower)
    }
}

impl Markdownify for PowerRange {
    fn markdownify(&self) -> String {
        format!("{:?}", self).to_case(Case::Lower)
    }
}

impl Markdownify for Power {
    fn markdownify(&self) -> String {
        let mut comps = Vec::new();
        comps.push(format!("**{}**.", capitalise(self.name.clone())));
        comps.push(format!(
            "_{}_, _{}_.",
            capitalise(self.range.markdownify()),
            self.duration.markdownify()
        ));
        comps.push(self.descr.clone());
        for enhancement in self.enhancements.iter() {
            comps.push(format!("    * {} EP -- {}", enhancement.0, enhancement.1));
        }
        comps.join("\n")
    }
}

impl Markdownify for Career {
    fn markdownify(&self) -> String {
        let mut comps = Vec::new();

        comps.push(format!("{}", self.descr));

        comps.push(format!(
            "**Skills**: {}.",
            self.skills
                .iter()
                .map(|x| format!("_{x}_"))
                .collect::<Vec<String>>()
                .join(", ")
        ));

        let money_str = if self.money > 0 {
            format!(", {}ʂ", self.money)
        } else {
            String::new()
        };

        let item_map = count_map(&self.items);
        comps.push(format!(
            "**Items**: {}{}.",
            item_map
                .iter()
                .map(|(x, c)| {
                    if *c > 1 {
                        format!("{}× {}", c, x.markdownify())
                    } else {
                        x.markdownify()
                    }
                })
                .collect::<Vec<String>>()
                .join(", "),
            money_str
        ));

        if !self.followers.is_empty() {
            comps.push(format!(
                "**Followers**: {}.",
                self.followers
                    .iter()
                    .map(|x| x.markdownify())
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
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

        // Name and cost.
        let cost_string = match self.cost {
            Some(cost) => format!(" ({}ʂ)", cost),
            None => String::new(),
        };
        comps.push(format!(
            "**{}**{}.",
            capitalise(self.name.clone()),
            cost_string
        ));

        // Abilities.
        comps.push(format!(
            "**STR:** {}, **AGI:** {}, **WIT:** {}.",
            self.str, self.agi, self.wit
        ));

        // Secondary properties.
        if self.size != CharacterSize::Medium {
            comps.push(format!("**Size:** {}.", self.size.markdownify()));
        }

        if let Some(_) = self.skills.iter().find(|x| *x == "tough") {
            comps.push(format!("**Health:** {}.", self.str + 3));
        }

        if self.mana > 0 {
            comps.push(format!("**Mana:** {}.", self.mana));
        }

        if self.omens > 0 {
            comps.push(format!("**Omens:** {}.", self.omens));
        }

        // // Armour value.
        // let mut armour = 0;
        // if let Some((_, nat_armour)) = self.natural_armour {
        //     armour = armour.max(nat_armour);
        // }
        // for item in self.items.iter() {
        //     if item.name == "heavy armour" {
        //         armour = armour.max(2);
        //         break;
        //     }
        //     if item.name == "light armour" {
        //         armour = armour.max(1);
        //         break;
        //     }
        // }

        // let shield_str = if self.items.iter().find(|x| x.name == "shield").is_some()
        // {     format!(" +shield")
        // } else {
        //     String::new()
        // };

        // if armour > 0 || !shield_str.is_empty() {
        //     comps.push(format!("*Armour:* {}{}.", armour, shield_str));
        // }

        // Skills and traits.
        let mut traits = self.traits.clone();
        if let Some((nat_arm_descr, nat_arm)) = &self.natural_armour {
            traits.push(Trait {
                name: String::from("armour"),
                descr: format!("{nat_arm_descr}, {nat_arm}"),
            });
        }
        for (nat_weapon, stats) in self.natural_weapons.iter() {
            traits.push(Trait {
                name: String::from("weapon"),
                descr: format!("{}, {}", nat_weapon, stats.markdownify()),
            });
        }
        if !traits.is_empty() {
            comps.push(format!(
                "**Traits:** {}.",
                traits
                    .iter()
                    .map(|x| { x.markdownify() })
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }

        if !self.skills.is_empty() {
            comps.push(format!(
                "**Skills:** {}.",
                self.skills
                    .iter()
                    .map(|x| format!("_{x}_"))
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }

        // Items.
        let money_str = if self.money > 0 {
            format!(", {}ʂ", self.money)
        } else {
            String::new()
        };
        if !self.items.is_empty() {
            comps.push(format!(
                "**Items:** {}{}.",
                self.items
                    .iter()
                    .map(|x| x.markdownify())
                    .collect::<Vec<String>>()
                    .join(", "),
                money_str
            ));
        }

        // Special rules
        for (n, d) in self.special_traits.iter() {
            comps.push(format!("\\\n**{}:** {}", capitalise(n.to_string()), d));
        }

        // Description.
        if !self.descr.is_empty() {
            let descr = self.descr.clone().replace("\n", " ");
            let descr = descr.trim();
            comps.push(format!("\\\n_{}_\n", descr));
        }

        comps.join(" ")
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
    let mut file = File::create(dir.join(format!("{tag}.md")))?;

    let die_size = items.len() / columns;

    write!(&mut file, "|D{die_size}\\D{columns}|")?;
    for i in 1..=columns {
        write!(&mut file, "{i}|")?;
    }
    write!(&mut file, "\n")?;

    for _ in 1..=(columns + 1) {
        write!(&mut file, "|:-:")?;
    }
    write!(&mut file, "|\n")?;

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

fn generate_rules() -> Result<(), Box<dyn Error>> {
    let base_path = Path::new("../gen");

    {
        let items: BTreeMap<String, Vec<Item>> =
            serde_yaml::from_reader(File::open("../game_data/items.yml")?)?;
        for (category, items) in items {
            let filename = format!("ref_{}.md", category.to_case(Case::Snake));
            let mut f = File::create(base_path.join(filename))?;
            for item in items {
                writeln!(f, "* {}\n", item.markdownify())?;
            }
        }
    }

    {
        let characters: BTreeMap<String, Vec<Character>> =
            serde_yaml::from_reader(File::open("../game_data/characters.yml")?)?;
        for (category, characters) in characters {
            let filename = format!("ref_{}.md", category.to_case(Case::Snake));
            let mut f = File::create(base_path.join(filename))?;
            for character in characters {
                writeln!(f, "{}\n", character.markdownify())?;
            }
        }
    }

    {
        let careers: Vec<Career> =
            serde_yaml::from_reader(File::open("../game_data/careers.yml")?)?;

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

        gen_double_die_table(&base_path, "Careers", &career_names, 4)?;
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

            let filename = format!("ref_{}.md", category.to_case(Case::Snake));
            let mut f = File::create(base_path.join(filename))?;
            for power in powers {
                writeln!(f, "* {}\n", power.markdownify())?;
            }

            gen_double_die_table(&base_path, &header, &power_names, power_names.len() / 6)?;
        }
    }

    {
        let skills: BTreeMap<String, String> =
            serde_yaml::from_reader(File::open("../game_data/skills.yml")?)?;
        let mut f = File::create(base_path.join("ref_skills.md"))?;
        for (name, descr) in skills {
            writeln!(f, "* **{}**.\n{}\n", capitalise(name), descr)?;
        }
    }

    {
        let traits: BTreeMap<String, String> =
            serde_yaml::from_reader(File::open("../game_data/traits.yml")?)?;
        let mut f = File::create(base_path.join("ref_traits.md"))?;
        for (name, descr) in traits {
            writeln!(f, "* **{}**.\n{}\n", capitalise(name), descr)?;
        }
    }

    {
        let conditions: BTreeMap<String, String> =
            serde_yaml::from_reader(File::open("../game_data/conditions.yml")?)?;
        let mut f = File::create(base_path.join("ref_conditions.md"))?;
        for (name, descr) in conditions {
            writeln!(f, "* **{}**.\n{}\n", capitalise(name), descr)?;
        }
    }

    {
        let description: BTreeMap<String, Vec<String>> =
            serde_yaml::from_reader(File::open("../game_data/description.yml")?)?;
        for (category, entries) in description {
            let columns = if entries.len() == 20 || entries.len() == 40 {
                2
            } else {
                4
            };
            gen_double_die_table(&base_path, &capitalise(category), &entries, columns)?;
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
                    comps.push(x.0.markdownify());
                    if x.1 > 0 {
                        comps.push(format!("{}ʂ", x.1));
                    }
                    comps.join(", ")
                })
                .collect::<Vec<String>>();
            gen_double_die_table(&base_path, &capitalise(category), &entries, 3)?;
        }
    }

    Ok(())
}

fn process_keywords_for_js(mut s: String) -> String {
    let re = Regex::new(r"_(?<s>[^_]*)_").unwrap();
    re.replace_all(&mut s, r"<i>${s}</i>").to_string()
}

pub fn generate_javascript() -> Result<(), Box<dyn Error>> {
    let output_dir = Path::new("../assets/js");
    let mut f = File::create(output_dir.join("data.js"))?;

    {
        let careers: Vec<Career> =
            serde_yaml::from_reader(File::open("../game_data/careers.yml")?)?;

        writeln!(f, "export const careers = [")?;
        for career in careers {
            let mana = career.mana;
            let skills = career.skills;
            let money = career.money;

            let mut items = Vec::new();
            let mut followers = Vec::new();
            let mut profane_scrolls = 0;
            let mut sacred_scrolls = 0;

            for item in career.items {
                if item.name == "power scroll" {
                    if item.descr == "profane" {
                        profane_scrolls += 1;
                    } else if item.descr == "sacred" {
                        sacred_scrolls += 1;
                    } else {
                        panic!("Unsupported scroll type");
                    }
                } else {
                    items.push(item.markdownify())
                }
            }

            for follower in career.followers {
                followers.push(follower.markdownify())
            }

            writeln!(f, "  {{")?;
            writeln!(
                f,
                "    masculine_name: \"{}\",",
                career.male_name.to_case(Case::Title)
            )?;
            writeln!(
                f,
                "    feminine_name: \"{}\",",
                career.female_name.to_case(Case::Title)
            )?;
            writeln!(
                f,
                "    description: \"{}\",",
                process_keywords_for_js(career.descr)
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
                    process_keywords_for_js(item.markdownify()),
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

    // The following hack is necessary to correctly generate the search index in the
    // careers chapter.
    let content_path = Path::new("../content/rules/careers.md");
    let mut content_file = File::create(content_path)?;
    writeln!(&content_file, "---")?;
    writeln!(&content_file, "title: Careers")?;
    writeln!(&content_file, "weight: 40")?;
    writeln!(&content_file, "prev: /rules/player_characters")?;
    writeln!(&content_file, "next: /rules/skills_traits_and_conditions")?;
    writeln!(&content_file, "---")?;
    writeln!(&content_file, "")?;

    let gen_path = Path::new("../gen/ref_careers.md");
    let gen_content = read_to_string(gen_path)?;
    content_file.write_all(gen_content.as_bytes())?;

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
    fn careers() {
        let careers: Vec<Career> =
            serde_yaml::from_reader(File::open("../game_data/careers.yml").unwrap()).unwrap();
        let skills = all_skills();
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
                    assert!(traits.contains(&t.name), "Invalid trait {}", t.name);
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
        let starting_items: BTreeMap<String, Vec<(AssetWithDescr, i32)>> =
            serde_yaml::from_reader(File::open("../game_data/starting_assets.yml").unwrap())
                .unwrap();
        let items = all_items();
        for (_, starting_items) in starting_items {
            for (item, _) in starting_items {
                assert!(items.contains(&item.name), "Invalid item {}", item.name);
            }
        }
    }
}
