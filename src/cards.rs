#[derive(Debug)]
enum CardSuit {
    A = 0,
    B,
    C,
    D,
    E,
}

#[derive(Debug)]
enum CardValue {
    One = 1,
    Two,
    Three,
    Four,
    Five, 
    Six,
    Seven,
    Eight,
    Nine,
}

enum GameRoomState {
    WaitingRoom,
    BeginGame,
    GameComplete,
}
