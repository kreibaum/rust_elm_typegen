use super::ElmExport;

#[allow(dead_code)]
struct Card {
    suit: String,
    value: u64,
}

#[allow(dead_code)]
enum Action {
    PlayCard(Card),
    DiscardCards(Vec<Card>),
    Surrender,
}

#[allow(dead_code)]
struct GameState {
    deck: Vec<Card>,
    discard_pile: Vec<Card>,
    replay_history: Vec<Action>,
}

impl ElmExport for Card {}
impl ElmExport for Action {}
impl ElmExport for GameState {}
