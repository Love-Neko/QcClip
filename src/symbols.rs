use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[repr(u32)]
pub(crate) enum SymbolGroupId {
    Recent,
    HappyCelebrate,
    EncouragePraise,
    GratefulFriendly,
    ComfortEmpathy,
    CuteShy,
    FunnyTeasing,
    DailyRest,
    AnimalMascot,
    ThinkingAnalyze,
    SurprisedShock,
    HelplessSigh,
    ApologyBow,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct SymbolGroupDefinition {
    pub(crate) id: SymbolGroupId,
    pub(crate) label_key: &'static str,
    pub(crate) icon: &'static str,
}

pub(crate) const SYMBOL_GROUPS: &[SymbolGroupDefinition] = &[
    SymbolGroupDefinition {
        id: SymbolGroupId::Recent,
        icon: "clock",
        label_key: "symbol.recent",
    },
    SymbolGroupDefinition {
        id: SymbolGroupId::HappyCelebrate,
        icon: "confetti",
        label_key: "kaomoji.happy_celebrate",
    },
    SymbolGroupDefinition {
        id: SymbolGroupId::EncouragePraise,
        icon: "thumb-up",
        label_key: "kaomoji.encourage_praise",
    },
    SymbolGroupDefinition {
        id: SymbolGroupId::GratefulFriendly,
        icon: "heart",
        label_key: "kaomoji.grateful_friendly",
    },
    SymbolGroupDefinition {
        id: SymbolGroupId::ComfortEmpathy,
        icon: "sun",
        label_key: "kaomoji.comfort_empathy",
    },
    SymbolGroupDefinition {
        id: SymbolGroupId::CuteShy,
        icon: "mood-bluz",
        label_key: "kaomoji.cute_shy",
    },
    SymbolGroupDefinition {
        id: SymbolGroupId::FunnyTeasing,
        icon: "mood-wink",
        label_key: "kaomoji.funny_teasing",
    },
    SymbolGroupDefinition {
        id: SymbolGroupId::DailyRest,
        icon: "coffee",
        label_key: "kaomoji.daily_rest",
    },
    SymbolGroupDefinition {
        id: SymbolGroupId::AnimalMascot,
        icon: "paw",
        label_key: "kaomoji.animal_mascot",
    },
    SymbolGroupDefinition {
        id: SymbolGroupId::ThinkingAnalyze,
        icon: "bulb",
        label_key: "kaomoji.thinking_analyze",
    },
    SymbolGroupDefinition {
        id: SymbolGroupId::SurprisedShock,
        icon: "mood-surprised",
        label_key: "kaomoji.surprised_shock",
    },
    SymbolGroupDefinition {
        id: SymbolGroupId::HelplessSigh,
        icon: "mood-confuzed",
        label_key: "kaomoji.helpless_sigh",
    },
    SymbolGroupDefinition {
        id: SymbolGroupId::ApologyBow,
        icon: "pray",
        label_key: "kaomoji.apology_bow",
    },
];

#[derive(Clone, Debug, serde::Deserialize)]
pub(crate) struct SymbolItem {
    pub(crate) glyph: String,
    pub(crate) name: String,
    pub(crate) search_text: String,
    pub(crate) category: SymbolGroupId,
}

pub(crate) struct SymbolData {
    pub(crate) items: Vec<SymbolItem>,
}

pub(crate) fn data() -> &'static SymbolData {
    static DATA: OnceLock<SymbolData> = OnceLock::new();
    DATA.get_or_init(|| {
        let items: Vec<SymbolItem> =
            serde_json::from_str(include_str!("../assets/data/kaomoji/kaomoji.json"))
                .expect("Failed to parse kaomoji.json");
        SymbolData { items }
    })
}

#[cfg(test)]
mod tests {
    use super::{SymbolGroupId, data};

    #[test]
    fn loads_classified_kaomoji() {
        let data = data();
        assert_eq!(data.items.len(), 823);
        assert!(
            data.items
                .iter()
                .any(|item| item.category == SymbolGroupId::HappyCelebrate)
        );
        assert!(
            data.items
                .iter()
                .any(|item| item.category == SymbolGroupId::AnimalMascot)
        );
    }
}
