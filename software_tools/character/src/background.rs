use convert_case::{Case, Casing};
use strum_macros::EnumIter;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Advancement {
    Skill(Skill),
    Abilities(Abilities),
    Mana,
}

impl From<Skill> for Advancement {
    fn from(value: Skill) -> Self {
        Self::Skill(value)
    }
}

impl From<Abilities> for Advancement {
    fn from(value: Abilities) -> Self {
        Self::Abilities(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Background {
    Acrobat,
    Agitator,
    Artisan,
    Assassin,
    BarberSurgeon,
    Beggar,
    Berserker,
    Boatman,
    Bodyguard,
    BonePicker,
    BountyHunter,
    Brawler,
    Burglar,
    Charlatan,
    CoachDriver,
    Cutpurse,
    Duellist,
    FireEater,
    Footpad,
    Gambler,
    HedgeWizard,
    Herbalist,
    Hunter,
    Jester,
    Knight,
    Labourer,
    Messenger,
    Miner,
    Noble,
    Outlaw,
    Peasant,
    Pedlar,
    Physician,
    PitFighter,
    Priest,
    Raconteur,
    RatCatcher,
    Scholar,
    Sharpshooter,
    Slayer,
    Soldier,
    Soothsayer,
    Thug,
    TombRobber,
    Townsman,
    WitchHunter,
    Wizard,
    Zealot,
}

static_gen_fn_all!(Background);

impl Background {
    pub fn gender_specific_name(&self, gender: Gender) -> String {
        match self {
            Self::Townsman => match gender {
                Gender::Male => self.to_string(),
                Gender::Female => String::from("townswoman"),
            },
            Self::HedgeWizard => match gender {
                Gender::Male => self.to_string(),
                Gender::Female => String::from("hedge witch"),
            },
            Self::BountyHunter => match gender {
                Gender::Male => self.to_string(),
                Gender::Female => String::from("bounty huntress"),
            },
            Self::Boatman => match gender {
                Gender::Male => self.to_string(),
                Gender::Female => String::from("boatwoman"),
            },
            Self::Hunter => match gender {
                Gender::Male => self.to_string(),
                Gender::Female => String::from("huntress"),
            },
            Self::Wizard => match gender {
                Gender::Male => self.to_string(),
                Gender::Female => String::from("witch"),
            },
            Self::Priest => match gender {
                Gender::Male => self.to_string(),
                Gender::Female => String::from("priestess"),
            },
            Self::WitchHunter => match gender {
                Gender::Male => self.to_string(),
                Gender::Female => String::from("witch huntress"),
            },
            _ => self.to_string(),
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Charlatan => {
                "You are a cunning liar, able to convince people of almost anything."
            }
            Self::Thug => {
                "A couple of blows with a heavy club are the quickest way to send the right \
                 message."
            }
            Self::Berserker => {
                "You are a savage, bloodthirsty warrior, barely able to control your bloodlust."
            }
            Self::Outlaw => {
                "The hills and woods are teeming with people escaping the law such as yourself."
            }
            Self::Townsman => {
                "Townsfolk such as yourself are the backbone of the Kingdom: shop owners, traders, \
                 local officials..."
            }
            Self::Bodyguard => {
                "You worked for the rich and powerful, protecting them from robbers and other scum."
            }
            Self::Herbalist => {
                "You were a village healer, curing ailments with plants and natural remedies."
            }
            Self::Acrobat => {
                "You earned your fare performing spectacular and dangerous acrobatic feats."
            }
            Self::Beggar => {
                "You spent your days sitting in the corner of busy roads, hoping for a few \
                 shillings from generous passers-by."
            }
            Self::Miner => {
                "Crawling through dark, cramped tunnels breathing dust was not the life you \
                 deserve."
            }
            Self::HedgeWizard => {
                "You were blessed with great power, but never received the education necessary to \
                 fully control it."
            }
            Self::Knight => {
                "With your honour lost and no coin left to your name, you wander on your steed \
                 selling your sword for money."
            }
            Self::BountyHunter => {
                "You used to roam the cities and the countryside, capturing wanted criminals to \
                 collect the bounties on their head."
            }
            Self::Burglar => "There is no lock you can't open and no safe you can't crack.",
            Self::Assassin => "For the right amount of coins, everyone can be taken care of.",
            Self::Physician => {
                "You are an erudite doctor, knowledgeable about poison, disease, antidotes, and \
                 cures."
            }
            Self::Agitator => {
                "You know how to incite the people with fervent speeches. You were partly \
                 responsible for the bloody riots of 1461: you swear you did it for your ideals, \
                 not money."
            }
            Self::Artisan => {
                "You know how to keep equipment in good order: nothing is more dangerous than a \
                 blunt sword or a cracked shield."
            }
            Self::BarberSurgeon => {
                "A sharp razor and a steady hand are required for both a clean shave and delicate \
                 surgeries."
            }
            Self::Boatman => {
                "You have ferried passengers and smuggled goods along the many rivers flowing \
                 across the Kingdom."
            }
            Self::BonePicker => {
                "One person's trash is another person's treasure: you scour the roads, graves, and \
                 gutters looking for something valuable to sell for a few pennies."
            }
            Self::CoachDriver => {
                "You worked for a coaching company, driving staging coaches through the dangerous \
                 roads of the Kingdom and protecting them with your trusty blunderbuss."
            }
            Self::Cutpurse => {
                "Some are born with too much: those wealthy nobs aren't going to notice if they \
                 are a few shillings short."
            }
            Self::Soldier => {
                "You have fought for the Kingdom and seen the horrors of war, the nightmares will \
                 never stop."
            }
            Self::Duellist => {
                "You are a skilled duellist, constantly looking for a worthy opponent to hone your \
                 skills."
            }
            Self::FireEater => {
                "You were a performer at a travelling carnival, executing a vast array of exciting \
                 tricks."
            }
            Self::Footpad => {
                "You have spent years mugging unaware victims in shady alleys and dark forest \
                 roads, occasionally killing for money."
            }
            Self::Gambler => {
                "Luck come and goes at the gaming table but it has never abandoned you, not with \
                 the help of a few clever tricks."
            }
            Self::Hunter => {
                "When you hunt in the dark forests of the Kingdom, you sometimes wonder if you \
                 really are the hunter or rather the prey."
            }
            Self::Jester => {
                "You have spent your life making a fool of yourself to entertain the nobles, but \
                 you will have the last laugh!"
            }
            Self::Brawler => {
                "Your fists are your weapon and your arms can bend iron: nobody is stronger than \
                 you!"
            }
            Self::Labourer => {
                "Your bones and muscles have been hardened by years of toiling under sun and rain."
            }
            Self::Wizard => {
                "You are a secretive scholar of the esoteric arts: many fear you, and with good \
                 reason."
            }
            Self::Messenger => {
                "Time is of the essence when carrying messages across the Kingdom, and you sure \
                 are a fast runner."
            }
            Self::Peasant => {
                "Your life was simple: growing crops and tending to livestock, trying to put \
                 enough food on the table to survive another winter."
            }
            Self::Pedlar => {
                "You made a small fortune transporting and trading exotic goods, but you lost \
                 everything because of a bad business decision."
            }
            Self::PitFighter => {
                "You have fought for money in illegal arenas and as a judicial champion for hire."
            }
            Self::Priest => {
                "Yours is the burden to teach and guide people so that they don't succumb to the \
                 lure of darkness."
            }
            Self::Raconteur => {
                "You have travelled far and wide across the kingdom, singing songs and act out \
                 enthralling stories."
            }
            Self::RatCatcher => {
                "Rats are everywhere and nobody likes them. You offer your services to get rid of \
                 them, but you swear they are getting bigger and nastier by the day..."
            }
            Self::Noble => {
                "Your house has fallen and you must now mingle with the lowly scum, but the day \
                 will come when you can reclaim what's yours by birthright!"
            }
            Self::Scholar => {
                "Hunched over dusty ancient tomes, you have accumulated vast amounts of knowledge: \
                 time to put it into practice!"
            }
            Self::Sharpshooter => {
                "Your skill with a bow or a gun has no equals: you can shoot a moving squirrel \
                 from half a mile away."
            }
            Self::Slayer => {
                "There is good pay for slaying giant monsters: it's a dangerous job, but you are \
                 brave and foolish enough to do it."
            }
            Self::Soothsayer => {
                "You are cursed with the ability to see what others can't, and have witnessed the \
                 end of the world."
            }
            Self::TombRobber => {
                "Precious treasures are buried in ancient crypts and old tombs: their previous \
                 owners aren't going to miss them."
            }
            Self::WitchHunter => {
                "Warlocks, witches, and sorcerers are a threat to mankind: they are destined to \
                 meet their end in the flames of a pyre."
            }
            Self::Zealot => {
                "You have a dark past and many sins to atone for: you are going to save your soul \
                 by purging the heretic!"
            }
        }
    }

    pub fn advancements(&self) -> [Advancement; 2] {
        match self {
            Self::Charlatan => [
                Advancement::from(Skill::Blather),
                Advancement::from(Skill::Charm),
            ],
            Self::Thug => [
                Advancement::from(Skill::Brawling),
                Advancement::from(Skill::Intimidate),
            ],
            Self::Berserker => [
                Advancement::from(Skill::BattleFrenzy),
                Advancement::from(Skill::Cleave),
            ],
            Self::Outlaw => [
                Advancement::from(Skill::Bushcraft),
                Advancement::from(Skill::SkilledShot),
            ],
            Self::Townsman => [
                Advancement::from(Skill::Bargain),
                Advancement::from(Skill::Gossip),
            ],
            Self::Bodyguard => [
                Advancement::from(Skill::Protect),
                Advancement::from(Skill::ShieldMastery),
            ],
            Self::Herbalist => [
                Advancement::from(Skill::Apothecary),
                Advancement::from(Skill::Heal),
            ],
            Self::Acrobat => [
                Advancement::from(Skill::Acrobatics),
                Advancement::from(Skill::Contortionist),
            ],
            Self::Beggar => [
                Advancement::from(Skill::DiseaseResistance),
                Advancement::from(Skill::Frugal),
            ],
            Self::Miner => [
                Advancement::from(Skill::Climb),
                Advancement::from(Skill::PiercingStrike),
            ],
            Self::HedgeWizard => [
                Advancement::from(Skill::Augury),
                Advancement::from(Skill::Sorcery),
            ],
            Self::Knight => [
                Advancement::from(Skill::Ride),
                Advancement::from(Skill::SkilledBlow),
            ],
            Self::BountyHunter => [
                Advancement::from(Skill::Disarm),
                Advancement::from(Skill::StrikeToStun),
            ],
            Self::Burglar => [
                Advancement::from(Skill::Burglary),
                Advancement::from(Skill::PackRat),
            ],
            Self::Assassin => [
                Advancement::from(Skill::SneakAttack),
                Advancement::from(Skill::Stealth),
            ],
            Self::Physician => [
                Advancement::from(Skill::Apothecary),
                Advancement::from(Skill::Medicine),
            ],
            Self::Artisan => [
                Advancement::from(Skill::Craft),
                Advancement::from(Skill::PiercingStrike),
            ],
            Self::BarberSurgeon => [
                Advancement::from(Skill::Heal),
                Advancement::from(Skill::StrikeToInjure),
            ],
            Self::Agitator => [
                Advancement::from(Skill::Charm),
                Advancement::from(Skill::QuickDraw),
            ],
            Self::Boatman => [
                Advancement::from(Skill::Boatmanship),
                Advancement::from(Skill::Swim),
            ],
            Self::BonePicker => [
                Advancement::from(Skill::Frugal),
                Advancement::from(Skill::QuickDraw),
            ],
            Self::CoachDriver => [
                Advancement::from(Skill::DriveCarts),
                Advancement::from(Skill::Ride),
            ],
            Self::Cutpurse => [
                Advancement::from(Skill::Steal),
                Advancement::from(Skill::Stealth),
            ],
            Self::Soldier => [
                Advancement::from(Skill::Cleave),
                Advancement::from(Skill::SkilledBlow),
            ],
            Self::Duellist => [
                Advancement::from(Skill::Ambidextrous),
                Advancement::from(Skill::FastStrike),
            ],
            Self::FireEater => [
                Advancement::from(Skill::Contortionist),
                Advancement::from(Skill::FireEating),
            ],
            Self::Footpad => [
                Advancement::from(Skill::SneakAttack),
                Advancement::from(Skill::StrikeToStun),
            ],
            Self::Gambler => [
                Advancement::from(Skill::Luck),
                Advancement::from(Skill::PlayGames),
            ],
            Self::Hunter => [
                Advancement::from(Skill::Bushcraft),
                Advancement::from(Skill::Hunt),
            ],
            Self::Jester => [
                Advancement::from(Skill::Acrobatics),
                Advancement::from(Skill::Blather),
            ],
            Self::Brawler => [
                Advancement::from(Skill::Brawling),
                Advancement::from(Skill::Wrestling),
            ],
            Self::Labourer => [
                Advancement::from(Skill::PackRat),
                Advancement::from(Skill::Tough),
            ],
            Self::Wizard => [Advancement::from(Skill::Sorcery), Advancement::Mana],
            Self::Messenger => [
                Advancement::from(Skill::Languages),
                Advancement::from(Skill::FleetFooted),
            ],
            Self::Peasant => [
                Advancement::from(Skill::AnimalHandling),
                Advancement::from(Skill::Tough),
            ],
            Self::Pedlar => [
                Advancement::from(Skill::Bargain),
                Advancement::from(Skill::Gossip),
            ],
            Self::PitFighter => [
                Advancement::from(Skill::DodgeBlows),
                Advancement::from(Skill::FastStrike),
            ],
            Self::Priest => [
                Advancement::from(Skill::Incorruptible),
                Advancement::from(Skill::Faith),
            ],
            Self::Raconteur => [
                Advancement::from(Skill::Act),
                Advancement::from(Skill::Music),
            ],
            Self::RatCatcher => [
                Advancement::from(Skill::DiseaseResistance),
                Advancement::from(Skill::PoisonResistance),
            ],
            Self::Noble => [
                Advancement::from(Skill::Leadership),
                Advancement::from(Skill::PoisonResistance),
            ],
            Self::Scholar => [
                Advancement::from(Skill::Alchemy),
                Advancement::from(Skill::Erudition),
            ],
            Self::Sharpshooter => [
                Advancement::from(Skill::SteadyAim),
                Advancement::from(Skill::SkilledShot),
            ],
            Self::Slayer => [
                Advancement::from(Skill::DodgeBlows),
                Advancement::from(Skill::MonsterSlaying),
            ],
            Self::Soothsayer => [
                Advancement::from(Skill::Augury),
                Advancement::from(Skill::Divination),
            ],
            Self::TombRobber => [
                Advancement::from(Skill::Burglary),
                Advancement::from(Skill::Climb),
            ],
            Self::WitchHunter => [
                Advancement::from(Skill::MagicSense),
                Advancement::from(Skill::MagicShield),
            ],
            Self::Zealot => [
                Advancement::from(Skill::BattleFrenzy),
                Advancement::from(Skill::Bravery),
            ],
        }
    }

    pub fn assets(&self) -> Vec<Asset> {
        match self {
            Self::Charlatan => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("fake city watch uniform, won't stand close scrutiny"),
                )),
                Asset::Item(Item::new(
                    None,
                    ItemKind::Scroll,
                    Some("filled with scribbled nonsense"),
                )),
                Asset::Item(Item::new(
                    Some("fake necklace"),
                    ItemKind::SilverJewel,
                    None,
                )),
            ],
            Self::Thug => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("covering your face"),
                )),
                Asset::Item(Item::from(ItemKind::AlcoholicDrink)),
                Asset::Item(Item::from(ItemKind::CrimsonWeed)),
            ],
            Self::Berserker => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("blood-splattered furs decorated with the skulls of your foes"),
                )),
                Asset::Item(Item::new(
                    Some("glaive"),
                    ItemKind::MartialGreatWeapon,
                    None,
                )),
            ],
            Self::Outlaw => vec![
                Asset::Item(Item::new(None, ItemKind::Clothes, Some("greenish cloak"))),
                Asset::Item(Item::new(Some("bow"), ItemKind::MissileGreatWeapon, None)),
            ],
            Self::Townsman => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("fashionable but practical"),
                )),
                Asset::Item(Item::from(ItemKind::Candle)),
                Asset::Item(Item::from(ItemKind::PipeAndTobacco)),
            ],
            Self::Bodyguard => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("enhancing your muscles"),
                )),
                Asset::Item(Item::from(ItemKind::Shield)),
                Asset::Item(Item::from(ItemKind::MedicineBox)),
            ],
            Self::Herbalist => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("many bags to carry herbs"),
                )),
                Asset::Item(Item::from(ItemKind::ApothecaryTools)),
                Asset::Item(Item::from(ItemKind::MedicineBox)),
            ],
            Self::Acrobat => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("comfortable and colourful leotard"),
                )),
                Asset::Item(Item::new(None, ItemKind::Rope, Some("made of silk"))),
                Asset::Item(Item::from(ItemKind::WoodenPole)),
            ],
            Self::Beggar => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("dirty, stinky rags"),
                )),
                Asset::Item(Item::from(ItemKind::Crutch)),
                Asset::Item(Item::from(ItemKind::AlcoholicDrink)),
            ],
            Self::Miner => vec![
                Asset::Item(Item::new(None, ItemKind::Clothes, Some("sweaty and dusty"))),
                Asset::Item(Item::new(
                    Some("pickaxe"),
                    ItemKind::SimpleGreatWeapon,
                    None,
                )),
                Asset::Item(Item::new(None, ItemKind::Cage, Some("containing a canary"))),
            ],
            Self::HedgeWizard => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("covered in bones, charms, and trinkets"),
                )),
                Asset::Item(Item::new(None, ItemKind::LuckyCharm, Some("rabbit foot"))),
                Asset::Item(Item::from(ItemKind::PowerScroll(
                    PowerContent::PowerOfKind(PowerKind::Profane),
                ))),
            ],
            Self::Knight => vec![
                Asset::Item(Item::new(None, ItemKind::Clothes, Some("colourful livery"))),
                Asset::Follower(Follower::new(
                    String::from("light horse"),
                    Some(format!(
                        "Bartadan, STR 12, AGI 4, WIT 8, ~{}~, trained for combat, d6 damage. Old \
                         and lame: lost the ~{}~ trait",
                        Trait::Sturdy,
                        Trait::Fast,
                    )),
                )),
            ],
            Self::BountyHunter => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("large coat and wide-brimmed hat"),
                )),
                Asset::Item(Item::from(ItemKind::Net)),
                Asset::Item(Item::from(ItemKind::ManaclesAndKey)),
            ],
            Self::Burglar => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("comfortable and with generous pockets"),
                )),
                Asset::Item(Item::from(ItemKind::LockPicks)),
                Asset::Item(Item::from(ItemKind::MetalFile)),
            ],
            Self::Assassin => vec![
                Asset::Item(Item::new(None, ItemKind::Clothes, Some("dark cloak"))),
                Asset::Item(Item::new(
                    Some("hand crossbow"),
                    ItemKind::MissileHandWeapon,
                    None,
                )),
                Asset::Item(Item::from(ItemKind::BlackAdderVenom)),
            ],
            Self::Physician => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("spotless black coat"),
                )),
                Asset::Item(Item::from(ItemKind::Antidote)),
                Asset::Item(Item::from(ItemKind::Cure)),
                Asset::Item(Item::from(ItemKind::DreamSand)),
            ],
            Self::Artisan => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("thick leather apron"),
                )),
                Asset::Item(Item::from(ItemKind::Toolbox)),
            ],
            Self::BarberSurgeon => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("bloodstained apron"),
                )),
                Asset::Item(Item::from(ItemKind::SurgeryTools)),
            ],
            Self::Boatman => vec![
                Asset::Item(Item::new(None, ItemKind::Clothes, Some("wet and mouldy"))),
                Asset::Item(Item::from(ItemKind::FishingTools)),
            ],
            Self::BonePicker => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("mismatched clothes scavenged from the road"),
                )),
                Asset::Item(Item::new(
                    Some("femur"),
                    ItemKind::SimpleHandWeapon,
                    Some("fragile"),
                )),
                Asset::Item(Item::from(ItemKind::BagOfDung)),
                Asset::Item(Item::from(ItemKind::Crutch)),
            ],
            Self::Agitator => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("working class attire"),
                )),
                Asset::Item(Item::from(ItemKind::AlchemistsFire)),
                Asset::Item(Item::new(
                    None,
                    ItemKind::Scroll,
                    Some(
                        "pamphlet denouncing the Emperor, or at least they told you so: you can't \
                         read after all",
                    ),
                )),
            ],
            Self::CoachDriver => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("coat and wide brimmed hat, for the rain"),
                )),
                Asset::Item(Item::new(Some("blunderbuss"), ItemKind::LongGun, None)),
            ],
            Self::Cutpurse => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("hood covering your face"),
                )),
                Asset::Item(Item::new(
                    None,
                    ItemKind::GoldJewel,
                    Some("gold necklace with a noble family emblem, stolen"),
                )),
            ],
            Self::Soldier => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("uniform from your regiment, has seen better days"),
                )),
                Asset::Item(Item::new(
                    Some("halberd"),
                    ItemKind::MartialGreatWeapon,
                    None,
                )),
            ],
            Self::Duellist => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("a few stitched up cuts"),
                )),
                Asset::Item(Item::new(
                    Some("thrusting sword"),
                    ItemKind::MartialHandWeapon,
                    None,
                )),
                Asset::Item(Item::new(
                    Some("parrying dagger"),
                    ItemKind::SimpleHandWeapon,
                    None,
                )),
            ],
            Self::FireEater => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("bare chested, lest your shirt catches fire"),
                )),
                Asset::Item(Item::new(
                    Some("throwing knives"),
                    ItemKind::MissileHandWeapon,
                    Some("you like to juggle with them"),
                )),
                Asset::Item(Item::from(ItemKind::AlcoholicDrink)),
                Asset::Item(Item::from(ItemKind::Torch)),
            ],
            Self::Footpad => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("scarf to cover your face"),
                )),
                Asset::Item(Item::new(
                    Some("cudgel"),
                    ItemKind::SimpleHandWeapon,
                    Some("ideal to knock people out"),
                )),
                Asset::Item(Item::from(ItemKind::Garrotte)),
            ],
            Self::Gambler => vec![
                Asset::Item(Item::new(None, ItemKind::Clothes, Some("large pockets"))),
                Asset::Item(Item::new(None, ItemKind::Cards, Some("marked"))),
                Asset::Item(Item::new(None, ItemKind::Dice, Some("loaded"))),
            ],
            Self::Hunter => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("decorated with trophies from your preys"),
                )),
                Asset::Item(Item::new(
                    Some("crossbow"),
                    ItemKind::MissileGreatWeapon,
                    None,
                )),
            ],
            Self::Jester => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("colourful, with a bell hat"),
                )),
                Asset::Item(Item::new(
                    Some("stick with bells"),
                    ItemKind::SimpleHandWeapon,
                    Some("jingles cheerfully when it hits someone"),
                )),
                Asset::Item(Item::from(ItemKind::SmokeBomb)),
            ],
            Self::Brawler => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("hand wraps to protect your knuckles"),
                )),
                Asset::Item(Item::from(ItemKind::AlcoholicDrink)),
                Asset::Item(Item::from(ItemKind::Chain)),
            ],
            Self::Labourer => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("drenched in sweat"),
                )),
                Asset::Item(Item::new(Some("shovel"), ItemKind::SimpleGreatWeapon, None)),
                Asset::Item(Item::new(None, ItemKind::Ration, Some("packed lunch"))),
            ],
            Self::Wizard => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("comfortable robes"),
                )),
                Asset::Item(Item::from(ItemKind::PowerScroll(PowerContent::Power(
                    Power::EldritchBlast,
                )))),
                Asset::Item(Item::from(ItemKind::PowerScroll(
                    PowerContent::PowerOfKind(PowerKind::Profane),
                ))),
            ],
            Self::Messenger => vec![
                Asset::Item(Item::new(None, ItemKind::Clothes, Some("dusty and sweaty"))),
                Asset::Item(Item::new(
                    None,
                    ItemKind::Scroll,
                    Some("sealed letter, no addressee"),
                )),
                Asset::Item(Item::from(ItemKind::ScrollCase)),
            ],
            Self::Peasant => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("stinking of manure"),
                )),
                Asset::Follower(Follower::new(
                    String::from("chicken"),
                    Some(format!(
                        "Bertha, STR 4, AGI 8, WIT 8, ~{}~, ~{}~ size, d4 damage. Dumb and brave",
                        Trait::Fast,
                        SizeCategory::Small
                    )),
                )),
                Asset::Follower(Follower::new(
                    String::from("pig"),
                    Some(String::from(
                        "Hans, STR 4, AGI 4, WIT 8, d4 damage. Picky about food",
                    )),
                )),
            ],
            Self::Pedlar => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("excessive amounts of cheap fake jewellery"),
                )),
                Asset::Item(Item::from(ItemKind::Darkroot)),
                Asset::Item(Item::from(ItemKind::Perfume)),
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("expensive, made of silk"),
                )),
            ],
            Self::PitFighter => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("torn, dusty, and covered in old blood"),
                )),
                Asset::Item(Item::new(Some("Spear"), ItemKind::MartialHandWeapon, None)),
                Asset::Item(Item::new(
                    Some("javelins"),
                    ItemKind::MissileHandWeapon,
                    None,
                )),
            ],
            Self::Priest => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("priestly robes and religious paraphernalia"),
                )),
                Asset::Item(Item::from(ItemKind::PowerScroll(
                    PowerContent::PowerOfKind(PowerKind::Sacred),
                ))),
                Asset::Item(Item::new(None, ItemKind::Book, Some("Holy Scriptures"))),
            ],
            Self::Raconteur => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("flamboyant and fashionable"),
                )),
                Asset::Item(Item::new(None, ItemKind::MusicInstrument, Some("fiddle"))),
            ],
            Self::RatCatcher => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("partly made of rat fur"),
                )),
                Asset::Item(Item::from(ItemKind::TrappingTools)),
                Asset::Follower(Follower::new(
                    String::from("small dog"),
                    Some(String::from(
                        "Brutus, trained to hunt rats, small but vicious",
                    )),
                )),
            ],
            Self::Noble => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("ostentatious fripperies, old and full of holes"),
                )),
                Asset::Item(Item::from(ItemKind::Mirror)),
                Asset::Item(Item::new(
                    None,
                    ItemKind::SignetRing,
                    Some("proof of your identity"),
                )),
            ],
            Self::Scholar => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("night gown and comfortable shoes"),
                )),
                Asset::Item(Item::from(ItemKind::QuillAndInk)),
                Asset::Item(Item::new(
                    None,
                    ItemKind::Book,
                    Some("blank, you can't wait to fill it with your learnings"),
                )),
                Asset::Item(Item::from(ItemKind::LookingGlass)),
            ],
            Self::Sharpshooter => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("wide brimmed hat to shade your eyes"),
                )),
                Asset::Item(Item::new(Some("arquebus"), ItemKind::LongGun, None)),
            ],
            Self::Slayer => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("thick hardened leather, cut, burnet, scratched, and torn"),
                )),
                Asset::Item(Item::new(
                    Some("huge battle axe"),
                    ItemKind::MartialGreatWeapon,
                    None,
                )),
            ],
            Self::Soothsayer => vec![
                Asset::Item(Item::new(None, ItemKind::Clothes, Some("hooded robes"))),
                Asset::Item(Item::from(ItemKind::DivinationTools)),
            ],
            Self::TombRobber => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("capacious knapsack to store the loot"),
                )),
                Asset::Item(Item::from(ItemKind::Crowbar)),
                Asset::Item(Item::from(ItemKind::Rope)),
            ],
            Self::WitchHunter => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("wide brimmed hat, pitch black clothes"),
                )),
                Asset::Item(Item::from(ItemKind::AlchemistsFire)),
                Asset::Item(Item::from(ItemKind::BlessedWater)),
                Asset::Item(Item::from(ItemKind::BlessedWater)),
            ],
            Self::Zealot => vec![
                Asset::Item(Item::new(
                    None,
                    ItemKind::Clothes,
                    Some("bloodstained monastic habit"),
                )),
                Asset::Item(Item::from(ItemKind::CrimsonWeed)),
                Asset::Item(Item::new(None, ItemKind::Book, Some("Holy Scriptures"))),
            ],
        }
    }
}

impl std::fmt::Display for Background {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BarberSurgeon => write!(f, "barber-surgeon"),
            _ => write!(f, "{}", format!("{self:?}").to_case(Case::Lower)),
        }
    }
}
