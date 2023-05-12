use crate::conditional_types::{Description, Headquarters, Name};

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Faction {
    pub(crate) symbol: FactionSymbol,
    pub(crate) name: Name,
    pub(crate) description: Description,
    pub(crate) headquarters: Headquarters,
    pub(crate) traits: Vec<Trait>,
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize, PartialEq, Eq)]
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

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Trait {
    pub(crate) symbol: FactionTraitSymbol,
    pub(crate) name: Name,
    pub(crate) description: Description,
}

#[derive(serde::Deserialize, Debug, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum FactionTraitSymbol {
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
