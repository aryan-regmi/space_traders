use crate::common::{Description, Headquarters, Name, NonEmptyString};

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Faction {
    symbol: FactionSymbol,
    name: Name,
    description: Description,
    headquarters: Headquarters,
    traits: Vec<Trait>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FactionSymbol {
    Cosmic,
    Void,
    Galactic,
    Quantum,
    Dominion,
}

impl FactionSymbol {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            FactionSymbol::Cosmic => "COSMIC",
            FactionSymbol::Void => "VOID",
            FactionSymbol::Galactic => "GALACTIC",
            FactionSymbol::Quantum => "QUANTUM",
            FactionSymbol::Dominion => "DOMINION",
        }
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Trait {
    symbol: TraitSymbol,
    name: Name,
    description: Description,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum TraitSymbol {
    Bureaucratic,
    Secretive,
    Capitalistic,
    Industrious,
    Peaceful,
    Distrustful,
    Welcoming,
    Anarchist,
    Conflicted,
    Authoritarian,
    Oligarchical,
    Dynastic,
    Democractic,
    Decentralized,
    Smugglers,
    Scavengers,
    Rebellious,
    Exiles,
    Pirates,
    Raiders,
    Clan,
    Guild,
    Dominion,
    Fringe,
    Forsaken,
    Isolated,
    Localized,
    Established,
    Notable,
    Dominant,
    Inescapable,
    Innovative,
    Bold,
    Visionary,
    Curious,
    Daring,
    Exploratory,
    Resourceful,
    Flexible,
    Cooperative,
    United,
    Strategic,
    Intelligent,
    RESEARCHfOCUSED,
    Collaborative,
    Progressive,
    Militaristic,
    TechnologicallyAdvanced,
    Aggressive,
    Imperialistic,
    TreasureHunters,
    Dexterous,
    Unpredictable,
    Brutal,
    Fleeting,
    Adaptable,
    SelfSufficient,
    Defensive,
    Proud,
    Diverse,
    Independent,
    SelfInterested,
    Fragmented,
    Commercial,
    FreeMarkets,
    Entrepreneurial,
}
