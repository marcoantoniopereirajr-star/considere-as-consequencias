use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;

pub const STORY_JSON_PT: &str = include_str!("../assets/story.json");
pub const STORY_JSON_EN: &str = include_str!("../assets/story_en.json");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Language {
    #[default]
    PtBr,
    En,
}

impl Language {
    pub fn toggle(self) -> Self {
        match self {
            Self::PtBr => Self::En,
            Self::En => Self::PtBr,
        }
    }

    pub fn badge(self) -> &'static str {
        match self {
            Self::PtBr => "PT-BR",
            Self::En => "EN 1930",
        }
    }

    pub fn t<'a>(self, pt: &'a str, en: &'a str) -> &'a str {
        match self {
            Self::PtBr => pt,
            Self::En => en,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookData {
    pub foreword: Foreword,
    pub nodes: HashMap<String, StoryNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Foreword {
    #[allow(dead_code)]
    pub title: String,
    pub paragraphs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoryNode {
    pub id: String,
    pub title: String,
    pub paragraphs: Vec<String>,
    pub prompt: Option<String>,
    #[serde(default)]
    pub choices: Vec<Choice>,
    #[serde(default)]
    pub is_ending: bool,
    pub ending_title: Option<String>,
    pub ending_tone: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    pub label: String,
    #[serde(default)]
    pub detail: String,
    pub target: String,
    #[serde(default)]
    pub trait_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Protagonist {
    Helen,
    Jed,
    Saunders,
}

impl Protagonist {
    pub const ALL: [Protagonist; 3] = [
        Protagonist::Helen,
        Protagonist::Jed,
        Protagonist::Saunders,
    ];

    pub fn from_node_id(id: &str) -> Self {
        if id.starts_with("J-") {
            Self::Jed
        } else if id.starts_with("S-") {
            Self::Saunders
        } else {
            Self::Helen
        }
    }

    pub fn prefix(self) -> &'static str {
        match self {
            Self::Helen => "H-",
            Self::Jed => "J-",
            Self::Saunders => "S-",
        }
    }

    pub fn start_id(self) -> &'static str {
        match self {
            Self::Helen => "H-0",
            Self::Jed => "J-0",
            Self::Saunders => "S-0",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Helen => "Helen Rogers",
            Self::Jed => "Jed Harringdale",
            Self::Saunders => "Saunders",
        }
    }

    pub fn chapter_label(self, lang: Language) -> &'static str {
        match (self, lang) {
            (Self::Helen, Language::PtBr) => "LIVRO I · HELEN",
            (Self::Jed, Language::PtBr) => "LIVRO II · JED",
            (Self::Saunders, Language::PtBr) => "LIVRO III · SAUNDERS",
            (Self::Helen, Language::En) => "BOOK I · HELEN",
            (Self::Jed, Language::En) => "BOOK II · JED",
            (Self::Saunders, Language::En) => "BOOK III · SAUNDERS",
        }
    }

    pub fn tagline(self, lang: Language) -> &'static str {
        match (self, lang) {
            (Self::Helen, Language::PtBr) => {
                "Entre os ditames do coração e a prudência da família na Nova Inglaterra."
            }
            (Self::Jed, Language::PtBr) => {
                "O herdeiro mimado diante da paixão, da fortuna e do seu próprio destino."
            }
            (Self::Saunders, Language::PtBr) => {
                "Quinhentos dólares, ambição e escolhas que podem mudar uma vida inteira."
            }
            (Self::Helen, Language::En) => {
                "Torn between the dictates of her heart and her New England family's prudence."
            }
            (Self::Jed, Language::En) => {
                "A wealthy widow's only son caught between passion, fortune, and his own fate."
            }
            (Self::Saunders, Language::En) => {
                "Five hundred dollars, ambition, and choices that can reshape an entire life."
            }
        }
    }

    pub fn roman(self) -> &'static str {
        match self {
            Self::Helen => "I",
            Self::Jed => "II",
            Self::Saunders => "III",
        }
    }
}

pub const ALL_TRAITS: [&str; 6] = [
    "coracao",
    "razao",
    "ousadia",
    "dever",
    "ambicao",
    "integridade",
];

pub fn trait_label(key: &str, lang: Language) -> &'static str {
    match (key, lang) {
        ("coracao", Language::PtBr) => "Coração",
        ("razao", Language::PtBr) => "Razão",
        ("ousadia", Language::PtBr) => "Ousadia",
        ("dever", Language::PtBr) => "Dever",
        ("ambicao", Language::PtBr) => "Ambição",
        ("integridade", Language::PtBr) => "Integridade",
        (_, Language::PtBr) => "Escolha",
        ("coracao", Language::En) => "Heart",
        ("razao", Language::En) => "Reason",
        ("ousadia", Language::En) => "Daring",
        ("dever", Language::En) => "Duty",
        ("ambicao", Language::En) => "Ambition",
        ("integridade", Language::En) => "Integrity",
        (_, Language::En) => "Choice",
    }
}

pub fn ending_tone_label(tone: &str, lang: Language) -> &'static str {
    match (tone, lang) {
        ("triunfo", Language::PtBr) => "TRIUNFO",
        ("sereno", Language::PtBr) => "SERENO",
        ("agridoce", Language::PtBr) => "AGRIDOCE",
        ("ironico", Language::PtBr) => "IRÔNICO",
        ("tragedia", Language::PtBr) => "TRAGÉDIA",
        (_, Language::PtBr) => "DESFECHO",
        ("triunfo", Language::En) => "TRIUMPH",
        ("sereno", Language::En) => "SERENE",
        ("agridoce", Language::En) => "BITTERSWEET",
        ("ironico", Language::En) => "IRONIC",
        ("tragedia", Language::En) => "TRAGEDY",
        (_, Language::En) => "ENDING",
    }
}

impl BookData {
    #[cfg(test)]
    pub fn load() -> Self {
        Self::load_for(Language::PtBr)
    }

    pub fn load_for(lang: Language) -> Self {
        let json_str = match lang {
            Language::PtBr => STORY_JSON_PT,
            Language::En => STORY_JSON_EN,
        };
        // Rename JSON field "trait" -> "trait_name" safely via raw deserialization
        #[derive(Deserialize)]
        struct RawBook {
            foreword: Foreword,
            nodes: HashMap<String, RawNode>,
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct RawNode {
            id: String,
            title: String,
            paragraphs: Vec<String>,
            prompt: Option<String>,
            #[serde(default)]
            choices: Vec<RawChoice>,
            #[serde(default)]
            is_ending: bool,
            ending_title: Option<String>,
            ending_tone: Option<String>,
        }
        #[derive(Deserialize)]
        struct RawChoice {
            label: String,
            #[serde(default)]
            detail: String,
            target: String,
            #[serde(rename = "trait", default)]
            trait_name: String,
        }

        let raw: RawBook =
            serde_json::from_str(json_str).expect("Falha ao carregar story JSON embutido");

        let nodes = raw
            .nodes
            .into_iter()
            .map(|(k, n)| {
                let choices = n
                    .choices
                    .into_iter()
                    .map(|c| Choice {
                        label: c.label,
                        detail: c.detail,
                        target: c.target,
                        trait_name: c.trait_name,
                    })
                    .collect();
                (
                    k,
                    StoryNode {
                        id: n.id,
                        title: n.title,
                        paragraphs: n.paragraphs,
                        prompt: n.prompt,
                        choices,
                        is_ending: n.is_ending,
                        ending_title: n.ending_title,
                        ending_tone: n.ending_tone,
                    },
                )
            })
            .collect();

        Self {
            foreword: raw.foreword,
            nodes,
        }
    }

    pub fn endings_for(&self, protagonist: Protagonist) -> Vec<&StoryNode> {
        let prefix = protagonist.prefix();
        let mut list: Vec<&StoryNode> = self
            .nodes
            .values()
            .filter(|n| n.id.starts_with(prefix) && n.is_ending)
            .collect();
        list.sort_by_key(|n| {
            n.id.split('-')
                .nth(1)
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0)
        });
        list
    }

    pub fn total_nodes_for(&self, protagonist: Protagonist) -> usize {
        let prefix = protagonist.prefix();
        self.nodes
            .keys()
            .filter(|k| k.starts_with(prefix))
            .count()
    }

    pub fn total_endings(&self) -> usize {
        self.nodes.values().filter(|n| n.is_ending).count()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub node_id: String,
    pub chosen_label: Option<String>,
    pub chosen_trait: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SaveState {
    pub current_node_id: Option<String>,
    pub history: Vec<HistoryEntry>,
    pub discovered_endings: HashSet<String>,
    pub visited_nodes: HashSet<String>,
    pub traits: BTreeMap<String, u32>,
    pub theme_index: usize,
    pub font_scale: f32,
    #[serde(default)]
    pub language: Language,
}

impl SaveState {
    pub fn new_default() -> Self {
        Self {
            current_node_id: None,
            history: Vec::new(),
            discovered_endings: HashSet::new(),
            visited_nodes: HashSet::new(),
            traits: BTreeMap::new(),
            theme_index: 0,
            font_scale: 1.0,
            language: Language::PtBr,
        }
    }

    fn save_path() -> PathBuf {
        if let Ok(exe) = std::env::current_exe()
            && let Some(dir) = exe.parent()
        {
            return dir.join("savegame.json");
        }
        PathBuf::from("savegame.json")
    }

    pub fn load_or_default() -> Self {
        let path = Self::save_path();
        if let Ok(content) = std::fs::read_to_string(&path)
            && let Ok(mut state) = serde_json::from_str::<SaveState>(&content)
        {
            if state.font_scale < 0.8 || state.font_scale > 1.4 {
                state.font_scale = 1.0;
            }
            return state;
        }
        Self::new_default()
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(Self::save_path(), json);
        }
    }

    pub fn discovered_endings_for(&self, protagonist: Protagonist) -> usize {
        let prefix = protagonist.prefix();
        self.discovered_endings
            .iter()
            .filter(|id| id.starts_with(prefix))
            .count()
    }

    pub fn has_any_progress(&self) -> bool {
        self.current_node_id.is_some()
            || !self.history.is_empty()
            || !self.discovered_endings.is_empty()
            || !self.visited_nodes.is_empty()
            || !self.traits.is_empty()
    }

    pub fn reset_progress(&mut self) {
        self.current_node_id = None;
        self.history.clear();
        self.discovered_endings.clear();
        self.visited_nodes.clear();
        self.traits.clear();
        self.save();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn loads_all_nodes_and_endings() {
        let book = BookData::load();
        assert_eq!(book.nodes.len(), 86, "Expected 86 total story nodes");
        assert_eq!(book.total_endings(), 43, "Expected 43 total endings");
    }

    #[test]
    fn start_nodes_exist() {
        let book = BookData::load();
        for start_id in ["H-0", "J-0", "S-0"] {
            assert!(
                book.nodes.contains_key(start_id),
                "Missing start node {start_id}"
            );
        }
    }

    #[test]
    fn all_choice_targets_exist() {
        let book = BookData::load();
        for (id, node) in &book.nodes {
            for choice in &node.choices {
                assert!(
                    book.nodes.contains_key(&choice.target),
                    "Node {id} has choice pointing to missing target {}",
                    choice.target
                );
            }
        }
    }

    #[test]
    fn ending_and_non_ending_choice_invariants() {
        let book = BookData::load();
        for (id, node) in &book.nodes {
            if node.is_ending {
                assert!(
                    node.choices.is_empty(),
                    "Ending node {id} should have 0 choices, found {}",
                    node.choices.len()
                );
            } else {
                assert!(
                    !node.choices.is_empty(),
                    "Non-ending node {id} should have at least 1 choice"
                );
            }
        }
    }

    #[test]
    fn bfs_reaches_all_endings_from_start_nodes() {
        let book = BookData::load();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        for start_id in ["H-0", "J-0", "S-0"] {
            visited.insert(start_id.to_string());
            queue.push_back(start_id.to_string());
        }

        while let Some(node_id) = queue.pop_front() {
            let node = book
                .nodes
                .get(&node_id)
                .unwrap_or_else(|| panic!("Node {node_id} not found during BFS"));
            for choice in &node.choices {
                if visited.insert(choice.target.clone()) {
                    queue.push_back(choice.target.clone());
                }
            }
        }

        let reachable_endings: Vec<&String> = book
            .nodes
            .iter()
            .filter(|(id, node)| node.is_ending && visited.contains(*id))
            .map(|(id, _)| id)
            .collect();

        assert_eq!(
            reachable_endings.len(),
            43,
            "Expected BFS from H-0, J-0, S-0 to reach all 43 endings"
        );
    }

    #[test]
    fn save_state_json_round_trip() {
        let mut state = SaveState::new_default();
        state.current_node_id = Some("H-1".to_string());
        state.history.push(HistoryEntry {
            node_id: "H-0".to_string(),
            chosen_label: Some("Seguir o coração".to_string()),
            chosen_trait: Some("coracao".to_string()),
        });
        state.discovered_endings.insert("H-14".to_string());
        state.visited_nodes.insert("H-0".to_string());
        state.visited_nodes.insert("H-1".to_string());
        state.traits.insert("coracao".to_string(), 2);
        state.theme_index = 1;
        state.font_scale = 1.15;

        let json = serde_json::to_string_pretty(&state).expect("Serialize SaveState");
        let restored: SaveState = serde_json::from_str(&json).expect("Deserialize SaveState");

        assert_eq!(restored.current_node_id, state.current_node_id);
        assert_eq!(restored.history.len(), 1);
        assert_eq!(restored.history[0].node_id, "H-0");
        assert_eq!(
            restored.history[0].chosen_label.as_deref(),
            Some("Seguir o coração")
        );
        assert_eq!(
            restored.history[0].chosen_trait.as_deref(),
            Some("coracao")
        );
        assert_eq!(restored.discovered_endings, state.discovered_endings);
        assert_eq!(restored.visited_nodes, state.visited_nodes);
        assert_eq!(restored.traits, state.traits);
        assert_eq!(restored.theme_index, state.theme_index);
        assert!((restored.font_scale - state.font_scale).abs() < f32::EPSILON);
    }

    #[test]
    fn save_state_reset_progress_preserves_preferences() {
        let save_path = SaveState::save_path();
        let previous_save = std::fs::read_to_string(&save_path).ok();

        let mut state = SaveState::new_default();
        state.current_node_id = Some("J-2".to_string());
        state.history.push(HistoryEntry {
            node_id: "J-0".to_string(),
            chosen_label: Some("Arriscar tudo".to_string()),
            chosen_trait: Some("ousadia".to_string()),
        });
        state.discovered_endings.insert("J-10".to_string());
        state.visited_nodes.insert("J-0".to_string());
        state.visited_nodes.insert("J-2".to_string());
        state.traits.insert("ousadia".to_string(), 3);
        state.theme_index = 2;
        state.font_scale = 1.25;

        assert!(state.has_any_progress());
        state.reset_progress();

        assert!(state.current_node_id.is_none());
        assert!(state.history.is_empty());
        assert!(state.discovered_endings.is_empty());
        assert!(state.visited_nodes.is_empty());
        assert!(state.traits.is_empty());
        assert!(!state.has_any_progress());
        assert_eq!(state.theme_index, 2);
        assert!((state.font_scale - 1.25).abs() < f32::EPSILON);

        // Restore workspace state so the test leaves no stray savegame.json behind.
        if let Some(prev) = previous_save {
            let _ = std::fs::write(&save_path, prev);
        } else {
            let _ = std::fs::remove_file(&save_path);
        }
    }

    #[test]
    fn english_book_matches_portuguese_graph_parity() {
        let pt = BookData::load_for(Language::PtBr);
        let en = BookData::load_for(Language::En);

        assert_eq!(en.nodes.len(), 86, "Expected 86 nodes in English edition");
        assert_eq!(en.total_endings(), 43, "Expected 43 endings in English edition");
        assert_eq!(pt.nodes.len(), en.nodes.len());

        for (id, pt_node) in &pt.nodes {
            let en_node = en
                .nodes
                .get(id)
                .unwrap_or_else(|| panic!("English edition missing node {id}"));
            assert_eq!(en_node.id, pt_node.id, "Node ID mismatch for {id}");
            assert_eq!(
                en_node.is_ending, pt_node.is_ending,
                "is_ending mismatch for {id}"
            );
            assert_eq!(
                en_node.ending_tone, pt_node.ending_tone,
                "ending_tone mismatch for {id}"
            );
            assert_eq!(
                en_node.choices.len(),
                pt_node.choices.len(),
                "choice count mismatch for {id}"
            );
            for (idx, (en_choice, pt_choice)) in
                en_node.choices.iter().zip(pt_node.choices.iter()).enumerate()
            {
                assert_eq!(
                    en_choice.target, pt_choice.target,
                    "choice[{idx}].target mismatch for {id}"
                );
                assert_eq!(
                    en_choice.trait_name, pt_choice.trait_name,
                    "choice[{idx}].trait_name mismatch for {id}"
                );
            }
        }
    }
}
