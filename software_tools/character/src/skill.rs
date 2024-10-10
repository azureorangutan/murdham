use convert_case::{Case, Casing};
use strum_macros::EnumIter;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Skill {
    Acrobatics,
    Act,
    Alchemy,
    Ambidextrous,
    AnimalHandling,
    Apothecary,
    Augury,
    Bargain,
    BattleFrenzy,
    Blather,
    Boatmanship,
    Bravery,
    Brawling,
    Burglary,
    Bushcraft,
    Charm,
    Cleave,
    Climb,
    Contortionist,
    Craft,
    Disarm,
    DiseaseResistance,
    Divination,
    DodgeBlows,
    DriveCarts,
    Erudition,
    Faith,
    FastStrike,
    FireEating,
    FleetFooted,
    Frugal,
    Gossip,
    Heal,
    Hunt,
    Incorruptible,
    Intimidate,
    Languages,
    Leadership,
    Luck,
    MagicSense,
    MagicShield,
    Medicine,
    MonsterSlaying,
    Music,
    PackRat,
    PiercingStrike,
    PlayGames,
    PoisonResistance,
    Protect,
    QuickDraw,
    Ride,
    ShieldMastery,
    SkilledBlow,
    SkilledShot,
    SneakAttack,
    Sorcery,
    SteadyAim,
    Steal,
    Stealth,
    StrikeToInjure,
    StrikeToStun,
    Swim,
    Tough,
    Wrestling,
}

static_gen_fn_all!(Skill);

impl Skill {
    pub fn description(&self) -> String {
        match self {
            // Movement
            Self::FleetFooted => format!(
                "When you sprint, you can move by an additional zone. You must only make an AGI \
                 save to sprint every two rounds (at the start of the 3rd, 5th, 7th... rounds). \
                 You can freely dodge attacks of opportunity without needing to spend your turn.",
            ),
            Self::Swim => format!(
                "Reduce difficulty or sprint while swimming.  Your attacks aren't impaired while \
                 in water. You can hold your breath for twice as long."
            ),
            Self::Climb => format!(
                "Reduce difficulty or sprint while climbing. Your attacks aren't impaired while \
                 climbing."
            ),
            Self::Acrobatics => format!(
                "Reduce difficulty or sprint while balancing. Reduce difficulty while leaping. \
                 When you fall, you reduce the falling distance by 4 metres, if you pass the AGI \
                 save, and by 2 metres even if you fail."
            ),

            Self::DriveCarts => format!(
                "Reduce difficulty while driving a cart. When your cart makes a full movement in \
                 combat, you can still use your main action to attack, but only after the \
                 movement is completed.",
            ),
            Self::Boatmanship => format!(
                "You count as two people when rowing a boat and know how to sail. When you \
                 forage, on a ~fish~ result you find an additional ~{}~, even if you don't have \
                 ~{}~.",
                ItemKind::Ration,
                ItemKind::FishingTools
            ),
            Self::Ride => format!(
                "Reduce difficulty while riding a tamed beast, ride without a ~{}~, or ride an \
                 untamed beast. When your mount makes a full movement in combat, you can still \
                 use your main action to attack, but only after the movement is completed.",
                ItemKind::Saddle
            ),

            Self::Contortionist => format!(
                "Your joints are extremely flexible and you can bend your body in absurd shapes. \
                 You can squeeze through small openings and easily escape bonds.",
            ),

            // Stealth and alertness
            Self::Burglary => format!(
                "Reduce difficulty when opening a lock, do it without tools (~{}~ or ~{}~), do it \
                 silently, or do it in only a round. You can react to all traps you trigger, even \
                 if you weren't aware of them.",
                ItemKind::Crowbar,
                ItemKind::LockPicks
            ),
            Self::Stealth => format!(
                "Reduce difficulty or sprint while sneaking. When your group is detected by \
                 another group, make an AGI save: if you pass you still managed to conceal \
                 yourself."
            ),
            Self::Steal => format!(
                "Reduce difficulty while stealing, or attempt to steal an item with bulk 1 \
                 instead of ½.",
            ),

            // Knowledge
            Self::Erudition => format!(
                "You can read, write, and perform advanced calculations. You can speak and \
                 understand Classic, the language of scholars and the Church. You are an expert \
                 in all manners of academic lore: history, geography, beasts, plants, philosophy, \
                 mathematics, astronomy, etc. The GM might give you additional information when \
                 it makes sense."
            ),
            Self::Languages => format!(
                "You can read and write. Each time you encounter a new language in your \
                 adventures there is chance you actually know it: 1:2 for common languages, 1:4 \
                 for dead or remote languages. Keep track of the languages you can speak."
            ),
            Self::Alchemy => format!(
                "You can read and write. You can speak and understand Classic, the language of \
                 scholars and the Church. You can craft alchemical substances, such as ~{}s~, \
                 ~{}s~, ~{}~, and ~{}s~. This requires raw materials (worth ¼ of the item) and an \
                 alchemist's workshop. You can craft a batch of 2 consumables of the same type in \
                 a watch.",
                ItemKind::AcidVial,
                ItemKind::AlchemistsFire,
                ItemKind::FlashPowder,
                ItemKind::SmokeBomb,
            ),
            Self::Medicine => format!(
                "You can read and write. You can speak and understand Classic, the language of \
                 scholars and the Church. You can diagnose poison and disease by spending a round \
                 examining a victim. After diagnosing, you can instruct someone with the ~{}~ \
                 skill to create a bespoke ~{}~ or ~{}~ which always works against that specific \
                 poison or disease.",
                Skill::Apothecary,
                ItemKind::Antidote,
                ItemKind::Cure,
            ),
            Self::Apothecary => format!(
                "You can craft medicinal substances, such as ~{}~, ~{}~, ~{}~, ~{}~, ~{}~, ~{}~, \
                 and all kinds of poison. This requires suitable ingredients and ~{}~. Creating a \
                 single dose takes only a stretch of time, but ingredients are rare. It takes a \
                 watch to gather them in the wilderness (if they are locally present), or to find \
                 them for sale in a settlement (they are worth ¼ the value of the end product).",
                ItemKind::Antidote,
                ItemKind::Cure,
                ItemKind::Darkroot,
                ItemKind::HealingDraught,
                ItemKind::MadcapMushroom,
                ItemKind::MedicineBox,
                ItemKind::ApothecaryTools
            ),
            Self::Heal => format!(
                "When you use a ~{}~: heal half STR in a stretch, or d8 damage (up to half STR) \
                 as a main action. When you use ~{}~, you are automatically successful without \
                 needing to pass a WIT save. You can attempt surgeries without tools, but you \
                 need to pass a WIT save then.",
                ItemKind::MedicineBox,
                ItemKind::SurgeryTools,
            ),

            // Social
            Self::AnimalHandling => format!(
                "You know how to take care of animals: feeding, grooming, taming, training, \
                 recognising signs of discomfort, etc. You can befriend wild animals by offering \
                 food ~and~ passing a WIT save, and domesticated animals by offering food ~or~ \
                 passing a WIT save. Befriended animals follow you until the end of the watch or \
                 you leave the area where they live. You can't befriend hostile animals, and you \
                 can only be accompanied by one befriended animal at a time.",
            ),
            Self::Charm => format!(
                "Reduce difficulty when befriending or persuading people. If you spend a stretch \
                 chatting or observing someone, you can estimate if they are bribable and how \
                 much they might want.",
            ),
            Self::Blather => format!(
                "You are able to speak endless strings of nonsense, leaving others dumbfounded. \
                 Reduce difficulty when distracting and taunting people."
            ),
            Self::Gossip => format!(
                "You might hear interesting rumours when taking a day rest or full rest in a \
                 settlement. The GM decides what you hear, and it isn't necessarily true. You can \
                 easily find contacts, even illegal ones such as fences, by spending a watch \
                 asking around for information.",
            ),
            Self::Bargain => format!(
                "If the buyer has this skill and the seller doesn't, goods are sold at half \
                 price. You are able to estimate the value of most items just by examining them.",
            ),
            Self::Intimidate => format!("Reduce difficulty when intimidating or torturing."),
            Self::Leadership => format!(
                "Reduce difficulty while inspiring and keeping the loyalty of followers. Once per \
                 stretch, as a main action, you can rally all ~{}~ and ~{}~ allies within range \
                 2. They make a group WIT save and those who succeed recover immediately.",
                Condition::Frightened,
                Condition::Terrified
            ),
            Self::Act => format!(
                "You are able to convincingly fake emotions and to disguise your voice and \
                 accent. This might give you an advantage in suitable social interactions or can \
                 be used to complement a disguise.",
            ),
            Self::Music => format!(
                "You know how to sing and play music instruments. During a day rest you can play \
                 an inspiring song for your party: all companions have a 1:4 chance of recovering \
                 1 spent omen."
            ),

            // Magic
            Self::Augury => format!(
                "You can spend a stretch to consult the entrails of a dead medium-sized animal to \
                 gain an omen. Sometimes, the entrails might provide a useful piece of \
                 information, at the GM's discretion.",
            ),
            Self::Divination => format!(
                "If you have ~{}~, you can spend an omen and a stretch of time to ask a question \
                 pertaining your current situation. The GM describes a vision giving you a \
                 cryptic answer. There is a 1:4 chance that the vision is wrong or misleading, \
                 rolled secretly by the GM.",
                ItemKind::DivinationTools,
            ),
            Self::MagicSense => format!(
                "You can spend a stretch in meditation to sense the presence of magic phenomena \
                 (ongoing powers, demons, magical creatures, etc.) in your zone or in your sector \
                 (your choice). You can only detect if any magic phenomena is present in the \
                 area, but can't count them, locate them, or determine their nature.",
            ),
            Self::MagicShield => format!(
                "You can use an ancient esoteric technique to erect a magic shield around you. \
                 Activating or deactivating it takes a stretch spent in meditation, and it \
                 deactivates automatically if you are ~{}~ or fall asleep. Profane powers have a \
                 1:2 chance of not working on you, no matter if harmful or beneficial. Other \
                 targets aren't protected and sacred powers aren't affected. Sorcerers can spend \
                 1 enhancement point to ignore the shield.",
                Condition::Incapacitated,
            ),
            Self::Faith => format!(
                "You can read and write. You can speak and understand Classic, the language of \
                 scholars and the Church. You can invoke sacred powers."
            ),
            Self::Sorcery => format!(
                "You can read and write. You can speak and understand Magick, the language used \
                 to invoke profane powers. This language is too convoluted to be used to \
                 communicate, but is essential to use magic. You can invoke profane powers. You \
                 can increase your maximum mana by 1 instead of taking a normal advancement, up \
                 to 6 at most."
            ),

            // Combat
            Self::BattleFrenzy => format!(
                "You can become ~{}~ by spending a main action, or immediately when you suffer \
                 damage. The condition lasts until all enemies have been defeated. You can spend \
                 a main action to try to calm yourself by passing a WIT save. While you are \
                 frenzied, you recover 1 health for each enemy you kill.",
                Condition::Frenzied
            ),
            Self::SkilledBlow => format!(
                "You improve the damage die of melee attacks (but not unarmed attacks): d4 to d6, \
                 d6 to d8, d8 to d10, d10 to d12. You can't improve a d12. In case of blast \
                 attacks only one target takes increased damage.",
            ),
            Self::SkilledShot => format!(
                "You improve the damage die of ranged attacks: d4 to d6, d6 to d8, d8 to d10, d10 \
                 to d12. You can't improve a d12. In case of blast attacks only one target takes \
                 increased damage.",
            ),
            Self::SteadyAim => format!("You double the range of ranged attacks."),
            Self::Cleave => format!(
                "When you inflict critical damage or kill a target with a melee attack, you can \
                 immediately attack another target with the same weapon. You can do this at most \
                 once per turn, and not while countering.",
            ),
            Self::Brawling => format!(
                "Your unarmed attacks are not impaired and inflict d6 damage. Your armour value \
                 is increased by 1 against unarmed attacks.",
            ),
            Self::Wrestling => format!(
                "Targets of your grapple attacks can't resist with a STR save unless they also \
                 have this skill. You can only be grappled by characters with this skill."
            ),
            Self::Ambidextrous => format!(
                "You can use both hands equally well. Damage is not impaired when you attack with \
                 a weapon in your non-dominant hand. You can attack the same target with two \
                 weapons at once, rolling damage for both but only considering the higher roll.",
            ),
            Self::Disarm => format!(
                "Targets of your disarm attacks can't resist with a STR save unless they also \
                 have this skill. You can only be disarmed by characters with this skill."
            ),
            Self::SneakAttack => format!(
                "You always inflict d12 damage when you attack unaware targets, no matter what \
                 weapons you use or if you are unarmed. Unarmed attacks are still impaired."
            ),
            Self::StrikeToStun => format!(
                "When you attack with a blunt weapon (a cudgel, the pommel of a sword, a rock...) \
                 you may choose to inflict no damage. You must still roll the damage die and \
                 compare the result with their current health. If damage beats or exceeds half \
                 the target's health, they are ~{}~ until the end of the stretch. if damage beats \
                 or exceeds the target's entire health, they are ~{}~ until the end of the watch. ",
                Condition::Incapacitated,
                Condition::Incapacitated
            ),
            Self::DodgeBlows => {
                format!("You can dodge for free, without spending your turn, once per round.")
            }
            Self::Protect => {
                format!("You can guard for free, without spending your turn, any number of times.")
            }
            Self::FastStrike => format!(
                "When you counter an attack or your attack is countered you always hit first \
                 unless your opponent also has this skill.",
            ),
            Self::MonsterSlaying => {
                format!("You inflict double damage against targets with larger size than you.")
            }
            Self::StrikeToInjure => format!(
                "When you inflict critical damage, you may choose to injure or kill the target. \
                 You choose what injury to apply instead of rolling on the table (it must still \
                 make somewhat sense), and you may choose that it is permanent rather than \
                 temporary."
            ),
            Self::PiercingStrike => format!(
                "If you roll higher than the target's armour value with an attack you ignore \
                 armour and inflict full damage. If you roll equal or lower, you inflict no \
                 damage as usual. This skill doesn't work in situations where you are required to \
                 pass a WIT save to hit.",
            ),
            Self::QuickDraw => {
                format!(
                    "You can equip and unequip any number of items held in hand as a single bonus \
                     action."
                )
            }
            Self::ShieldMastery => {
                format!(
                    "When you hold a shield, your armour value is increased by 1 against all \
                     attacks, not just if you react or are countered. If you are unaware of the \
                     attack, however, your shield still doesn't protect you."
                )
            }

            // Miscellanea
            Self::FireEating => format!(
                "You can consume ~{}~ and spit it through an open flame, such as a lit ~{}~, to \
                 make a melee blast attack inflicting d4 fire damage. You reduce incoming fire \
                 damage by 1.",
                ItemKind::AlcoholicDrink,
                ItemKind::Torch
            ),
            Self::Frugal => format!(
                "You don't reduce abilities when you can't satisfy needs during a day rest. \
                 However, you still have to satisfy them all in order to heal and recover mana. \
                 You pay half for lodging, since you have very low standards."
            ),
            Self::Luck => format!(
                "Your maximum omens are increased by 1. When you use an omen, there is a 1:4 \
                 chance it isn't actually spent. When choosing the target of an indiscriminate \
                 effect, such as a trap or a monster ambush, the GM might prioritise other \
                 characters over you.",
            ),
            Self::PackRat => format!(
                "Your carry limit are increased by 2: you can carry up to 10 bulk unencumbered, \
                 and up to 20 encumbered. This also changes how heavy it is to carry you!"
            ),
            Self::PlayGames => format!(
                "You can learn to play games quickly: after you have played a game, you can't be \
                 beaten by others unless they also have this skill. You know how to cheat: your \
                 cheating attempts are always successful unless your opponents are paying close \
                 attention to you. People might still get suspicious if you win too much."
            ),
            Self::Craft => format!(
                "You can repair items without a ~{}~ or without having to pass a save to succeed \
                 You can craft non-consumable items, such as weapons, armour, and vehicles. This \
                 takes a watch and requires raw materials worth ¼ of the item and a workshop with \
                 all the necessary tools and equipment. Crafting items with the ~durability~ \
                 keyword takes one watch every 2 points of durability.",
                ItemKind::Toolbox
            ),
            Self::Bravery => format!("You are immune to fear and treat terror as fear",),
            Self::PoisonResistance => format!(
                "You are resistant to alcohol, poisons, and drugs. You ignore the first dose \
                 taken within a stretch: it has no effect. You can resist a second dose with a \
                 STR save, and a third dose works automatically.",
            ),
            Self::DiseaseResistance => format!(
                "You develop an immunity to all diseases you recover from and can't contract them \
                 again. Keep track of the diseases you are immune to."
            ),
            Self::Bushcraft => format!(
                "Pass a WIT save to ignore the movement penalty when travelling between sectors \
                 or regions without following a path. If you have ~{}~, you pass automatically. \
                 You can sleep in the wilderness without a ~{}~. When you forage, you roll twice \
                 on the foraging table and apply both results.",
                ItemKind::NavigationTools,
                ItemKind::CampingKit,
            ),
            Self::Hunt => format!(
                "Reduce difficulty when following trails. When you forage, on a ~small game~ \
                 result you find an additional ~{}~, even if you don't have ~{}~.",
                ItemKind::Ration,
                ItemKind::TrappingTools,
            ),
            Self::Incorruptible => {
                format!("You heal 1 corruption on a day rest and all corruption on a full rest.")
            }
            Self::Tough => {
                format!(
                    "Your maximum health, and the threshold for instant death, are increased by 2 \
                     (they equal your STR plus 2)."
                )
            }
        }
    }
}

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_case(Case::Lower))
    }
}
