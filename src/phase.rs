#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Phase {
    InitialPlacement {
        player: usize,
        placing_second: bool,
        placing_road: bool,
    },
    Turn {
        player: usize,
        turn_phase: TurnPhase,
        development_phase: DevelopmentPhase,
    },
    FinishedGame {
        winner: usize,
    },
}

impl Phase {
    pub const START_GAME: Phase = Phase::InitialPlacement {
        player: 0,
        placing_second: false,
        placing_road: false,
    };
    pub const START_TURNS: Phase = Phase::Turn {
        player: 0,
        turn_phase: TurnPhase::PreRoll,
        development_phase: DevelopmentPhase::Ready,
    };

    pub fn player(&self) -> usize {
        match self {
            Phase::InitialPlacement {
                player,
                placing_second: _,
                placing_road: _,
            } => *player,
            Phase::Turn {
                player: _,
                turn_phase: TurnPhase::Discard(player),
                development_phase: _,
            } => *player,
            Phase::Turn {
                player,
                turn_phase: _,
                development_phase: _,
            } => *player,
            Phase::FinishedGame { winner } => *winner,
        }
    }
    pub fn is_turn(&self) -> bool {
        if let Phase::Turn {
            player: _,
            turn_phase: _,
            development_phase: _,
        } = self
        {
            true
        } else {
            false
        }
    }

    pub fn is_thief(&self) -> bool {
        if let Phase::Turn {
            player: _,
            turn_phase,
            development_phase,
        } = self
        {
            *turn_phase == TurnPhase::MoveThief
                || *development_phase == DevelopmentPhase::KnightActive
        } else {
            false
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TurnPhase {
    PreRoll,
    Discard(usize),
    MoveThief,
    Free,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DevelopmentPhase {
    Ready,
    KnightActive,
    RoadBuildingActive { two_left: bool },
    YearOfPlentyActive { two_left: bool },
    DevelopmentPlayed,
}

impl TurnPhase {
    pub fn unbound(&self) -> bool {
        match *self {
            TurnPhase::PreRoll | TurnPhase::Free => true,
            _ => false,
        }
    }

    pub fn is_discard(&self) -> bool {
        match *self {
            TurnPhase::Discard(_) => true,
            _ => false,
        }
    }
}
