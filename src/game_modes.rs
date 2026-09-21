use crate::models;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Modalities {
    HumanVsComputer,
    ComputerVsComputer,
    ComputerVsHuman,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AILevel {
    VeryEasy,
    Easy,
    Medium,
    Hard,
    VeryHard,
    Extreme,
    MinmaxVeryEasy,
    MinmaxEasy,
    MinmaxNormal,
    MinmaxHard,
    MinmaxVeryHard,
    MinmaxExtreme,
}

pub fn ai_level_from_str(s: &str) -> Option<AILevel> {
    match s {
        "VeryEasy" => Some(AILevel::VeryEasy),
        "Easy" => Some(AILevel::Easy),
        "Medium" => Some(AILevel::Medium),
        "Hard" => Some(AILevel::Hard),
        "VeryHard" => Some(AILevel::VeryHard),
        "Extreme" => Some(AILevel::Extreme),
        "minmax-very-easy" | "MinmaxVeryEasy" | "MinimaxVeryEasy" => Some(AILevel::MinmaxVeryEasy),
        "minmax-easy" | "MinmaxEasy" | "MinimaxEasy" => Some(AILevel::MinmaxEasy),
        "minmax-normal" | "MinmaxNormal" | "MinimaxNormal" => Some(AILevel::MinmaxNormal),
        "minmax-hard" | "MinmaxHard" | "MinimaxHard" => Some(AILevel::MinmaxHard),
        "minmax-very-hard" | "MinmaxVeryHard" | "MinimaxVeryHard" => Some(AILevel::MinmaxVeryHard),
        "minmax-extreme" | "MinmaxExtreme" | "MinimaxExtreme" => Some(AILevel::MinmaxExtreme),
        _ => None,
    }
}

pub fn ai_moves(ai_level: AILevel) -> usize {
    match ai_level {
        AILevel::VeryEasy => 80,
        AILevel::Easy => 150,
        AILevel::Medium => 500,
        AILevel::Hard => 2000,
        AILevel::VeryHard => 5000,
        AILevel::Extreme => 10000,
        AILevel::MinmaxVeryEasy
        | AILevel::MinmaxEasy
        | AILevel::MinmaxNormal
        | AILevel::MinmaxHard
        | AILevel::MinmaxVeryHard
        | AILevel::MinmaxExtreme => 0,
    }
}

pub fn minmax_depth(ai_level: AILevel) -> Option<usize> {
    match ai_level {
        AILevel::MinmaxVeryEasy => Some(2),
        AILevel::MinmaxEasy => Some(3),
        AILevel::MinmaxNormal => Some(4),
        AILevel::MinmaxHard => Some(5),
        AILevel::MinmaxVeryHard => Some(6),
        AILevel::MinmaxExtreme => Some(7),
        _ => None,
    }
}

pub fn ai_level_label(ai_level: AILevel) -> &'static str {
    match ai_level {
        AILevel::VeryEasy => "Very Easy",
        AILevel::Easy => "Easy",
        AILevel::Medium => "Medium",
        AILevel::Hard => "Hard",
        AILevel::VeryHard => "Very Hard",
        AILevel::Extreme => "Extreme",
        AILevel::MinmaxVeryEasy => "Minmax Very Easy",
        AILevel::MinmaxEasy => "Minmax Easy",
        AILevel::MinmaxNormal => "Minmax Normal",
        AILevel::MinmaxHard => "Minmax Hard",
        AILevel::MinmaxVeryHard => "Minmax Very Hard",
        AILevel::MinmaxExtreme => "Minmax Extreme",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minmax_levels_and_assigns_depths() {
        assert_eq!(
            ai_level_from_str("minmax-normal"),
            Some(AILevel::MinmaxNormal)
        );
        assert_eq!(minmax_depth(AILevel::MinmaxNormal), Some(4));
        assert_eq!(ai_level_from_str("MinimaxEasy"), Some(AILevel::MinmaxEasy));
        assert_eq!(ai_level_label(AILevel::MinmaxHard), "Minmax Hard");
        assert_eq!(minmax_depth(AILevel::Extreme), None);
    }
}

pub fn get_opposite_from_turn(player: models::Player, modality: Modalities) -> models::Player {
    match player {
        models::Player::Player1 | models::Player::AIPlayer1 => models::Player::AIPlayer2,
        models::Player::Player2 => models::Player::AIPlayer1,
        models::Player::AIPlayer2 if modality == Modalities::HumanVsComputer => {
            models::Player::Player1
        }
        models::Player::AIPlayer2 if modality == Modalities::ComputerVsComputer => {
            models::Player::AIPlayer1
        }
        models::Player::AIPlayer2 if modality == Modalities::ComputerVsHuman => {
            models::Player::Player1
        }
        // ??? needed to compile
        _ => models::Player::AIPlayer2,
    }
}
