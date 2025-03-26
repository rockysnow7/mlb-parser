use std::cmp::Ordering;

use pyo3::pyclass;
use strum_macros::EnumIter;

#[derive(Debug)]
pub struct Weather {
    condition: String,
    temperature: u64,
    wind_speed: u64,
}

#[derive(Debug)]
pub struct Context {
    game_pk: u64,
    date: String,
    venue: String,
    weather: Weather,
}

#[derive(Clone, Copy, EnumIter, PartialEq, Eq, Debug)]
pub enum Position {
    Pitcher,
    Catcher,
    FirstBase,
    SecondBase,
    ThirdBase,
    Shortstop,
    LeftField,
    CenterField,
    RightField,
    DesignatedHitter,
    PinchHitter,
    PinchRunner,
    TwoWayPlayer,
    Outfield,
    Infield,
    Utility,
    ReliefPitcher,
    StartingPitcher,
}

impl ToString for Position {
    fn to_string(&self) -> String {
        match self {
            Position::Pitcher => "PITCHER",
            Position::Catcher => "CATCHER",
            Position::FirstBase => "FIRST_BASE",
            Position::SecondBase => "SECOND_BASE",
            Position::ThirdBase => "THIRD_BASE",
            Position::Shortstop => "SHORTSTOP",
            Position::LeftField => "LEFT_FIELD",
            Position::CenterField => "CENTER_FIELD",
            Position::RightField => "RIGHT_FIELD",
            Position::DesignatedHitter => "DESIGNATED_HITTER",
            Position::PinchHitter => "PINCH_HITTER",
            Position::PinchRunner => "PINCH_RUNNER",
            Position::TwoWayPlayer => "TWO_WAY_PLAYER",
            Position::Outfield => "OUTFIELD",
            Position::Infield => "INFIELD",
            Position::Utility => "UTILITY",
            Position::ReliefPitcher => "RELIEF_PITCHER",
            Position::StartingPitcher => "STARTING_PITCHER",
        }.to_string()
    }
}

impl std::str::FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PITCHER" => Ok(Position::Pitcher),
            "CATCHER" => Ok(Position::Catcher),
            "FIRST_BASE" => Ok(Position::FirstBase),
            "SECOND_BASE" => Ok(Position::SecondBase),
            "THIRD_BASE" => Ok(Position::ThirdBase),
            "SHORTSTOP" => Ok(Position::Shortstop),
            "LEFT_FIELD" => Ok(Position::LeftField),
            "CENTER_FIELD" => Ok(Position::CenterField),
            "RIGHT_FIELD" => Ok(Position::RightField),
            "DESIGNATED_HITTER" => Ok(Position::DesignatedHitter),
            "PINCH_HITTER" => Ok(Position::PinchHitter),
            "PINCH_RUNNER" => Ok(Position::PinchRunner),
            "TWO_WAY_PLAYER" => Ok(Position::TwoWayPlayer),
            "OUTFIELD" => Ok(Position::Outfield),
            "INFIELD" => Ok(Position::Infield),
            "UTILITY" => Ok(Position::Utility),
            "RELIEF_PITCHER" => Ok(Position::ReliefPitcher),
            "STARTING_PITCHER" => Ok(Position::StartingPitcher),
            _ => Err(format!("Invalid position: {}", s)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub position: Position,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Team {
    team_id: u64,
    players: Vec<Player>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopBottom {
    Top,
    Bottom,
}

impl ToString for TopBottom {
    fn to_string(&self) -> String {
        match self {
            TopBottom::Top => "top",
            TopBottom::Bottom => "bottom",
        }.to_string()
    }
}

impl std::str::FromStr for TopBottom {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "top" => Ok(TopBottom::Top),
            "bottom" => Ok(TopBottom::Bottom),
            _ => Err(format!("Invalid top/bottom: {}", s)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Inning {
    pub number: u64,
    pub top_bottom: TopBottom,
}

impl ToString for Inning {
    fn to_string(&self) -> String {
        format!("{} {}", self.number, self.top_bottom.to_string())
    }
}

pub enum BaseComparison {
    From,
    To,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Base {
    Home,
    First,
    Second,
    Third,
}

impl Base {
    pub fn compare(&self, other: &Base, comparison: BaseComparison) -> Ordering {
        match (comparison, self, other) {
            (BaseComparison::From, Base::Home, _) => Ordering::Less,
            (BaseComparison::From, _, Base::Home) => Ordering::Greater,
            (BaseComparison::To, Base::Home, _) => Ordering::Greater,
            (BaseComparison::To, _, Base::Home) => Ordering::Less,
            _ => self.cmp(other),
        }
    }
}

impl ToString for Base {
    fn to_string(&self) -> String {
        match self {
            Base::Home => "home".to_string(),
            Base::First => "1".to_string(),
            Base::Second => "2".to_string(),
            Base::Third => "3".to_string(),
        }
    }
}

impl std::str::FromStr for Base {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Base::First),
            "2" => Ok(Base::Second),
            "3" => Ok(Base::Third),
            "4" | "home" => Ok(Base::Home),
            _ => Err(format!("Invalid base: {}", s)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlayContent {
    Groundout {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    BuntGroundout {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    Strikeout {
        batter: String,
        pitcher: String,
    },
    Lineout {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    BuntLineout {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    Flyout {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    PopOut {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    BuntPopOut {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    Forceout {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    FieldersChoiceOut {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        scoring_runner: String,
    },
    DoublePlay {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    TriplePlay {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    RunnerDoublePlay {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    RunnerTriplePlay {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    GroundedIntoDoublePlay {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    StrikeoutDoublePlay {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    Pickoff {
        base: Base,
        fielders: Vec<String>,
        runner: String,
    },
    PickoffError {
        base: Base,
        fielders: Vec<String>,
        runner: String,
    },
    CaughtStealing {
        base: Base,
        fielders: Vec<String>,
        runner: String,
    },
    PickoffCaughtStealing {
        base: Base,
        fielders: Vec<String>,
        runner: String,
    },
    WildPitch {
        pitcher: String,
        runner: String,
    },
    RunnerOut {
        fielders: Vec<String>,
        runner: String,
    },
    FieldOut {
        fielders: Vec<String>,
        runner: String,
    },
    BatterOut {
        batter: String,
        catcher: String,
    },
    Balk {
        pitcher: String,
    },
    PassedBall {
        pitcher: String,
        catcher: String,
    },
    Error {
        pitcher: String,
        catcher: String,
    },
    Single {
        batter: String,
        pitcher: String,
    },
    Double {
        batter: String,
        pitcher: String,
    },
    Triple {
        batter: String,
        pitcher: String,
    },
    HomeRun {
        batter: String,
        pitcher: String,
    },
    Walk {
        batter: String,
        pitcher: String,
    },
    IntentWalk {
        batter: String,
        pitcher: String,
    },
    HitByPitch {
        batter: String,
        pitcher: String,
    },
    FieldersChoice {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    CatcherInterference {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    StolenBase {
        base: Base,
        scoring_runner: String,
    },
    SacFly {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        scoring_runner: String,
    },
    SacFlyDoublePlay {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        scoring_runner: String,
    },
    SacBunt {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        runner: String,
    },
    SacBuntDoublePlay {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
        runner: String,
    },
    FieldError {
        batter: String,
        pitcher: String,
        fielders: Vec<String>,
    },
    GameAdvisory,
}

#[derive(Clone, Copy, Debug, Hash, EnumIter, PartialEq, Eq)]
pub enum PlayType {
    Groundout,
    BuntGroundout,
    Strikeout,
    Lineout,
    BuntLineout,
    Flyout,
    PopOut,
    BuntPopOut,
    Forceout,
    FieldersChoiceOut,
    DoublePlay,
    TriplePlay,
    RunnerDoublePlay,
    RunnerTriplePlay,
    GroundedIntoDoublePlay,
    StrikeoutDoublePlay,
    Pickoff,
    PickoffError,
    CaughtStealing,
    PickoffCaughtStealing,
    WildPitch,
    RunnerOut,
    FieldOut,
    BatterOut,
    Balk,
    PassedBall,
    Error,
    Single,
    Double,
    Triple,
    HomeRun,
    Walk,
    IntentWalk,
    HitByPitch,
    FieldersChoice,
    CatcherInterference,
    StolenBase,
    SacFly,
    SacFlyDoublePlay,
    SacBunt,
    SacBuntDoublePlay,
    FieldError,
    GameAdvisory,
}

impl ToString for PlayType {
    fn to_string(&self) -> String {
        match self {
            PlayType::Groundout => "Groundout",
            PlayType::BuntGroundout => "Bunt Groundout",
            PlayType::Strikeout => "Strikeout",
            PlayType::Lineout => "Lineout",
            PlayType::BuntLineout => "Bunt Lineout",
            PlayType::Flyout => "Flyout",
            PlayType::PopOut => "Pop Out",
            PlayType::BuntPopOut => "Bunt Pop Out",
            PlayType::Forceout => "Forceout",
            PlayType::FieldersChoiceOut => "Fielders Choice Out",
            PlayType::DoublePlay => "Double Play",
            PlayType::TriplePlay => "Triple Play",
            PlayType::RunnerDoublePlay => "Runner Double Play",
            PlayType::RunnerTriplePlay => "Runner Triple Play",
            PlayType::GroundedIntoDoublePlay => "Grounded Into Double Play",
            PlayType::StrikeoutDoublePlay => "Strikeout Double Play",
            PlayType::Pickoff => "Pickoff",
            PlayType::PickoffError => "Pickoff Error",
            PlayType::CaughtStealing => "Caught Stealing",
            PlayType::PickoffCaughtStealing => "Pickoff Caught Stealing",
            PlayType::WildPitch => "Wild Pitch",
            PlayType::RunnerOut => "Runner Out",
            PlayType::FieldOut => "Field Out",
            PlayType::BatterOut => "Batter Out",
            PlayType::Balk => "Balk",
            PlayType::PassedBall => "Passed Ball",
            PlayType::Error => "Error",
            PlayType::Single => "Single",
            PlayType::Double => "Double",
            PlayType::Triple => "Triple",
            PlayType::HomeRun => "Home Run",
            PlayType::Walk => "Walk",
            PlayType::IntentWalk => "Intent Walk",
            PlayType::HitByPitch => "Hit By Pitch",
            PlayType::FieldersChoice => "Fielders Choice",
            PlayType::CatcherInterference => "Catcher Interference",
            PlayType::StolenBase => "Stolen Base",
            PlayType::SacFly => "Sac Fly",
            PlayType::SacFlyDoublePlay => "Sac Fly Double Play",
            PlayType::SacBunt => "Sac Bunt",
            PlayType::SacBuntDoublePlay => "Sac Bunt Double Play",
            PlayType::FieldError => "Field Error",
            PlayType::GameAdvisory => "Game Advisory",
        }.to_string()
    }
}

impl std::str::FromStr for PlayType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Groundout" => Ok(PlayType::Groundout),
            "Bunt Groundout" => Ok(PlayType::BuntGroundout),
            "Strikeout" => Ok(PlayType::Strikeout),
            "Lineout" => Ok(PlayType::Lineout),
            "Bunt Lineout" => Ok(PlayType::BuntLineout),
            "Flyout" => Ok(PlayType::Flyout),
            "Pop Out" => Ok(PlayType::PopOut),
            "Bunt Pop Out" => Ok(PlayType::BuntPopOut),
            "Forceout" => Ok(PlayType::Forceout),
            "Fielders Choice Out" => Ok(PlayType::FieldersChoiceOut),
            "Double Play" => Ok(PlayType::DoublePlay),
            "Triple Play" => Ok(PlayType::TriplePlay),
            "Runner Double Play" => Ok(PlayType::RunnerDoublePlay),
            "Runner Triple Play" => Ok(PlayType::RunnerTriplePlay),
            "Grounded Into Double Play" => Ok(PlayType::GroundedIntoDoublePlay),
            "Strikeout Double Play" => Ok(PlayType::StrikeoutDoublePlay),
            "Pickoff" => Ok(PlayType::Pickoff),
            "Pickoff Error" => Ok(PlayType::PickoffError),
            "Caught Stealing" => Ok(PlayType::CaughtStealing),
            "Pickoff Caught Stealing" => Ok(PlayType::PickoffCaughtStealing),
            "Wild Pitch" => Ok(PlayType::WildPitch),
            "Runner Out" => Ok(PlayType::RunnerOut),
            "Field Out" => Ok(PlayType::FieldOut),
            "Batter Out" => Ok(PlayType::BatterOut),
            "Balk" => Ok(PlayType::Balk),
            "Passed Ball" => Ok(PlayType::PassedBall),
            "Error" => Ok(PlayType::Error),
            "Single" => Ok(PlayType::Single),
            "Double" => Ok(PlayType::Double),
            "Triple" => Ok(PlayType::Triple),
            "Home Run" => Ok(PlayType::HomeRun),
            "Walk" => Ok(PlayType::Walk),
            "Intent Walk" => Ok(PlayType::IntentWalk),
            "Hit By Pitch" => Ok(PlayType::HitByPitch),
            "Fielders Choice" => Ok(PlayType::FieldersChoice),
            "Catcher Interference" => Ok(PlayType::CatcherInterference),
            "Stolen Base" => Ok(PlayType::StolenBase),
            "Sac Fly" => Ok(PlayType::SacFly),
            "Sac Fly Double Play" => Ok(PlayType::SacFlyDoublePlay),
            "Sac Bunt" => Ok(PlayType::SacBunt),
            "Sac Bunt Double Play" => Ok(PlayType::SacBuntDoublePlay),
            "Field Error" => Ok(PlayType::FieldError),
            "Game Advisory" => Ok(PlayType::GameAdvisory),
            _ => Err(format!("Invalid play type: {}", s)),
        }
    }
}

impl PlayType {
    pub fn requires_base(&self) -> bool {
        matches!(
            self,
            PlayType::Pickoff |
            PlayType::PickoffError |
            PlayType::CaughtStealing |
            PlayType::PickoffCaughtStealing |
            PlayType::StolenBase
        )
    }

    pub fn requires_batter(&self) -> bool {
        matches!(
            self,
            PlayType::Groundout |
            PlayType::BuntGroundout |
            PlayType::Strikeout |
            PlayType::Lineout |
            PlayType::BuntLineout |
            PlayType::Flyout |
            PlayType::PopOut |
            PlayType::BuntPopOut |
            PlayType::Forceout |
            PlayType::FieldersChoiceOut |
            PlayType::DoublePlay |
            PlayType::TriplePlay |
            PlayType::RunnerDoublePlay |
            PlayType::RunnerTriplePlay |
            PlayType::GroundedIntoDoublePlay |
            PlayType::StrikeoutDoublePlay |
            PlayType::BatterOut |
            PlayType::Single |
            PlayType::Double |
            PlayType::Triple |
            PlayType::HomeRun |
            PlayType::Walk |
            PlayType::IntentWalk |
            PlayType::HitByPitch |
            PlayType::FieldersChoice |
            PlayType::CatcherInterference |
            PlayType::SacFly |
            PlayType::SacFlyDoublePlay |
            PlayType::SacBunt |
            PlayType::SacBuntDoublePlay |
            PlayType::FieldError
        )
    }

    pub fn requires_pitcher(&self) -> bool {
        matches!(
            self,
            PlayType::Groundout |
            PlayType::BuntGroundout |
            PlayType::Strikeout |
            PlayType::Lineout |
            PlayType::BuntLineout |
            PlayType::Flyout |
            PlayType::PopOut |
            PlayType::BuntPopOut |
            PlayType::Forceout |
            PlayType::FieldersChoiceOut |
            PlayType::DoublePlay |
            PlayType::TriplePlay |
            PlayType::RunnerDoublePlay |
            PlayType::RunnerTriplePlay |
            PlayType::GroundedIntoDoublePlay |
            PlayType::StrikeoutDoublePlay |
            PlayType::WildPitch |
            PlayType::Balk |
            PlayType::PassedBall |
            PlayType::Error |
            PlayType::Single |
            PlayType::Double |
            PlayType::Triple |
            PlayType::HomeRun |
            PlayType::Walk |
            PlayType::IntentWalk |
            PlayType::HitByPitch |
            PlayType::FieldersChoice |
            PlayType::CatcherInterference |
            PlayType::SacFly |
            PlayType::SacFlyDoublePlay |
            PlayType::SacBunt |
            PlayType::SacBuntDoublePlay |
            PlayType::FieldError
        )
    }

    pub fn requires_catcher(&self) -> bool {
        matches!(
            self,
            PlayType::BatterOut |
            PlayType::PassedBall |
            PlayType::Error
        )
    }

    pub fn requires_fielders(&self) -> bool {
        matches!(
            self,
            PlayType::Groundout |
            PlayType::BuntGroundout |
            PlayType::Lineout |
            PlayType::BuntLineout |
            PlayType::Flyout |
            PlayType::PopOut |
            PlayType::BuntPopOut |
            PlayType::Forceout |
            PlayType::FieldersChoiceOut |
            PlayType::DoublePlay |
            PlayType::TriplePlay |
            PlayType::RunnerDoublePlay |
            PlayType::RunnerTriplePlay |
            PlayType::GroundedIntoDoublePlay |
            PlayType::StrikeoutDoublePlay |
            PlayType::Pickoff |
            PlayType::PickoffError |
            PlayType::CaughtStealing |
            PlayType::PickoffCaughtStealing |
            PlayType::RunnerOut |
            PlayType::FieldOut |
            PlayType::FieldersChoice |
            PlayType::CatcherInterference |
            PlayType::SacFly |
            PlayType::SacFlyDoublePlay |
            PlayType::SacBunt |
            PlayType::SacBuntDoublePlay |
            PlayType::FieldError
        )
    }

    pub fn requires_runner(&self) -> bool {
        matches!(
            self,
            PlayType::Pickoff |
            PlayType::PickoffError |
            PlayType::CaughtStealing |
            PlayType::PickoffCaughtStealing |
            PlayType::WildPitch |
            PlayType::RunnerOut |
            PlayType::FieldOut |
            PlayType::StolenBase |
            PlayType::SacBunt |
            PlayType::SacBuntDoublePlay
        )
    }

    pub fn requires_scoring_runner(&self) -> bool {
        matches!(
            self,
            PlayType::FieldersChoiceOut |
            PlayType::SacFly |
            PlayType::SacFlyDoublePlay
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Movement {
    pub runner: String,
    pub from: Base,
    pub to: Base,
    pub out: bool,
}

impl ToString for Movement {
    fn to_string(&self) -> String {
        format!(
            "{} {} -> {}{}",
            self.runner,
            self.from.to_string(),
            self.to.to_string(),
            if self.out { " [out]" } else { "" }
        )
    }
}

#[derive(Debug)]
pub struct MovementBuilder {
    runner: Option<String>,
    from: Option<Base>,
    to: Option<Base>,
    out: bool,
}

impl MovementBuilder {
    pub fn new() -> Self {
        Self { runner: None, from: None, to: None, out: false }
    }

    pub fn set_runner(&mut self, runner: String) -> &mut Self {
        self.runner = Some(runner);
        self
    }

    pub fn set_from(&mut self, from: Base) -> &mut Self {
        self.from = Some(from);
        self
    }

    pub fn set_to(&mut self, to: Base) -> &mut Self {
        self.to = Some(to);
        self
    }

    pub fn set_out(&mut self) -> &mut Self {
        self.out = true;
        self
    }

    pub fn build(&self) -> Result<Movement, String> {
        Ok(Movement {
            runner: self.runner.clone().ok_or("Runner is required, not set")?,
            from: self.from.clone().ok_or("From is required, not set")?,
            to: self.to.clone().ok_or("To is required, not set")?,
            out: self.out,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Play {
    pub inning: Inning,
    pub play_content: PlayContent,
    pub movements: Vec<Movement>,
}

pub struct PlayBuilder {
    pub inning: Option<Inning>,
    pub play_type: Option<PlayType>,
    pub base: Option<Base>,
    pub batter: Option<String>,
    pub pitcher: Option<String>,
    pub catcher: Option<String>,
    pub fielders: Vec<String>,
    pub runner: Option<String>,
    pub scoring_runner: Option<String>,
    pub movement_builder: MovementBuilder,
    pub movements: Vec<Movement>,
}

impl PlayBuilder {
    pub fn new() -> Self {
        Self {
            inning: None,
            play_type: None,
            base: None,
            batter: None,
            pitcher: None,
            catcher: None,
            fielders: Vec::new(),
            runner: None,
            scoring_runner: None,
            movement_builder: MovementBuilder::new(),
            movements: Vec::new(),
        }
    }

    pub fn set_inning(&mut self, inning: Inning) -> &mut Self {
        self.inning = Some(inning);
        self
    }

    pub fn set_play_type(&mut self, play_type: PlayType) -> &mut Self {
        self.play_type = Some(play_type);
        self
    }

    pub fn set_base(&mut self, base: Base) -> &mut Self {
        self.base = Some(base);
        self
    }

    pub fn set_batter(&mut self, batter: String) -> &mut Self {
        self.batter = Some(batter);
        self
    }

    pub fn set_pitcher(&mut self, pitcher: String) -> &mut Self {
        self.pitcher = Some(pitcher);
        self
    }

    pub fn set_catcher(&mut self, catcher: String) -> &mut Self {
        self.catcher = Some(catcher);
        self
    }

    pub fn add_fielder(&mut self, fielder: String) -> &mut Self {
        self.fielders.push(fielder);
        self
    }

    pub fn set_runner(&mut self, runner: String) -> &mut Self {
        self.runner = Some(runner);
        self
    }

    pub fn set_scoring_runner(&mut self, scoring_runner: String) -> &mut Self {
        self.scoring_runner = Some(scoring_runner);
        self
    }

    pub fn reset_movement_builder(&mut self) -> &mut Self {
        self.movement_builder = MovementBuilder::new();
        self
    }

    pub fn build_movement(&mut self) -> Result<&mut Self, String> {
        self.movements.push(self.movement_builder.build()?);
        self.reset_movement_builder();

        Ok(self)
    }

    pub fn build(&self) -> Option<Play> {
        let play_content = match self.play_type {
            Some(PlayType::Groundout) => PlayContent::Groundout {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::BuntGroundout) => PlayContent::BuntGroundout {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::Strikeout) => PlayContent::Strikeout {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
            },
            Some(PlayType::Lineout) => PlayContent::Lineout {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::BuntLineout) => PlayContent::BuntLineout {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::Flyout) => PlayContent::Flyout {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::PopOut) => PlayContent::PopOut {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::BuntPopOut) => PlayContent::BuntPopOut {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::Forceout) => PlayContent::Forceout {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::FieldersChoiceOut) => PlayContent::FieldersChoiceOut {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
                scoring_runner: self.scoring_runner.clone()?,
            },
            Some(PlayType::DoublePlay) => PlayContent::DoublePlay {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::TriplePlay) => PlayContent::TriplePlay {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::RunnerDoublePlay) => PlayContent::RunnerDoublePlay {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::RunnerTriplePlay) => PlayContent::RunnerTriplePlay {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::GroundedIntoDoublePlay) => PlayContent::GroundedIntoDoublePlay {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::StrikeoutDoublePlay) => PlayContent::StrikeoutDoublePlay {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::Pickoff) => PlayContent::Pickoff {
                base: self.base.clone()?,
                fielders: self.fielders.clone(),
                runner: self.runner.clone()?,
            },
            Some(PlayType::PickoffError) => PlayContent::PickoffError {
                base: self.base.clone()?,
                fielders: self.fielders.clone(),
                runner: self.runner.clone()?,
            },
            Some(PlayType::CaughtStealing) => PlayContent::CaughtStealing {
                base: self.base.clone()?,
                fielders: self.fielders.clone(),
                runner: self.runner.clone()?,
            },
            Some(PlayType::PickoffCaughtStealing) => PlayContent::PickoffCaughtStealing {
                base: self.base.clone()?,
                fielders: self.fielders.clone(),
                runner: self.runner.clone()?,
            },
            Some(PlayType::WildPitch) => PlayContent::WildPitch {
                pitcher: self.pitcher.clone()?,
                runner: self.runner.clone()?,
            },
            Some(PlayType::RunnerOut) => PlayContent::RunnerOut {
                fielders: self.fielders.clone(),
                runner: self.runner.clone()?,
            },
            Some(PlayType::FieldOut) => PlayContent::FieldOut {
                fielders: self.fielders.clone(),
                runner: self.runner.clone()?,
            },
            Some(PlayType::BatterOut) => PlayContent::BatterOut {
                batter: self.batter.clone()?,
                catcher: self.catcher.clone()?,
            },
            Some(PlayType::Balk) => PlayContent::Balk {
                pitcher: self.pitcher.clone()?,
            },
            Some(PlayType::PassedBall) => PlayContent::PassedBall {
                pitcher: self.pitcher.clone()?,
                catcher: self.catcher.clone()?,
            },
            Some(PlayType::Error) => PlayContent::Error {
                pitcher: self.pitcher.clone()?,
                catcher: self.catcher.clone()?,
            },
            Some(PlayType::Single) => PlayContent::Single {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
            },
            Some(PlayType::Double) => PlayContent::Double {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
            },
            Some(PlayType::Triple) => PlayContent::Triple {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
            },
            Some(PlayType::HomeRun) => PlayContent::HomeRun {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
            },
            Some(PlayType::Walk) => PlayContent::Walk {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
            },
            Some(PlayType::IntentWalk) => PlayContent::IntentWalk {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
            },
            Some(PlayType::HitByPitch) => PlayContent::HitByPitch {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
            },
            Some(PlayType::FieldersChoice) => PlayContent::FieldersChoice {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::CatcherInterference) => PlayContent::CatcherInterference {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::StolenBase) => PlayContent::StolenBase {
                base: self.base.clone()?,
                scoring_runner: self.scoring_runner.clone()?,
            },
            Some(PlayType::SacFly) => PlayContent::SacFly {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
                scoring_runner: self.scoring_runner.clone()?,
            },
            Some(PlayType::SacFlyDoublePlay) => PlayContent::SacFlyDoublePlay {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
                scoring_runner: self.scoring_runner.clone()?,
            },
            Some(PlayType::SacBunt) => PlayContent::SacBunt {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
                runner: self.scoring_runner.clone()?,
            },
            Some(PlayType::SacBuntDoublePlay) => PlayContent::SacBuntDoublePlay {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
                runner: self.scoring_runner.clone()?,
            },
            Some(PlayType::FieldError) => PlayContent::FieldError {
                batter: self.batter.clone()?,
                pitcher: self.pitcher.clone()?,
                fielders: self.fielders.clone(),
            },
            Some(PlayType::GameAdvisory) => PlayContent::GameAdvisory,
            None => return None,
        };

        Some(Play {
            inning: self.inning.clone()?,
            play_content,
            movements: self.movements.clone(),
        })
    }
}

#[pyclass]
#[derive(Debug)]
pub struct Game {
    context: Context,
    home_team: Team,
    away_team: Team,
    plays: Vec<Play>,
}

pub struct GameBuilder {
    pub game_pk: Option<u64>,
    pub date: Option<String>,
    pub venue: Option<String>,
    pub weather_condition: Option<String>,
    pub weather_temperature: Option<u64>,
    pub weather_wind_speed: Option<u64>,

    pub home_team_id: Option<u64>,
    pub home_team_players: Vec<Player>,

    pub away_team_id: Option<u64>,
    pub away_team_players: Vec<Player>,

    pub play_builder: PlayBuilder,
    pub plays: Vec<Play>,
}

impl GameBuilder {
    pub fn new() -> Self {
        Self {
            game_pk: None,
            date: None,
            venue: None,
            weather_condition: None,
            weather_temperature: None,
            weather_wind_speed: None,
            home_team_id: None,
            home_team_players: Vec::new(),
            away_team_id: None,
            away_team_players: Vec::new(),
            play_builder: PlayBuilder::new(),
            plays: Vec::new(),
        }
    }

    // context section methods
    pub fn set_game_pk(&mut self, game_pk: u64) -> &mut Self {
        self.game_pk = Some(game_pk);
        self
    }

    pub fn set_date(&mut self, date: String) -> &mut Self {
        self.date = Some(date);
        self
    }

    pub fn set_venue(&mut self, venue: String) -> &mut Self {
        self.venue = Some(venue);
        self
    }

    pub fn set_weather(&mut self, condition: String, temperature: u64, wind_speed: u64) -> &mut Self {
        self.weather_condition = Some(condition);
        self.weather_temperature = Some(temperature);
        self.weather_wind_speed = Some(wind_speed);
        self
    }

    // home team section methods
    pub fn set_home_team_id(&mut self, team_id: u64) -> &mut Self {
        self.home_team_id = Some(team_id);
        self
    }

    pub fn add_home_team_player(&mut self, player: Player) -> &mut Self {
        self.home_team_players.push(player);
        self
    }

    // away team section methods
    pub fn set_away_team_id(&mut self, team_id: u64) -> &mut Self {
        self.away_team_id = Some(team_id);
        self
    }

    pub fn add_away_team_player(&mut self, player: Player) -> &mut Self {
        self.away_team_players.push(player);
        self
    }

    // play section methods
    pub fn reset_play_builder(&mut self) -> &mut Self {
        self.play_builder = PlayBuilder::new();
        self
    }

    pub fn build_play(&mut self) -> Option<&mut Self> {
        self.plays.push(self.play_builder.build()?);
        self.reset_play_builder();

        Some(self)
    }

    // build method to create the final Game object
    pub fn build(&self) -> Option<Game> {
        // make sure we have all required fields
        let game_pk = self.game_pk?;
        let date = self.date.clone()?;
        let venue = self.venue.clone()?;
        let weather_condition = self.weather_condition.clone()?;
        let weather_temperature = self.weather_temperature?;
        let weather_wind_speed = self.weather_wind_speed?;
        let home_team_id = self.home_team_id?;
        let away_team_id = self.away_team_id?;

        // create the context
        let context = Context {
            game_pk,
            date,
            venue,
            weather: Weather {
                condition: weather_condition,
                temperature: weather_temperature,
                wind_speed: weather_wind_speed,
            },
        };

        // create teams
        let home_team = Team {
            team_id: home_team_id,
            players: self.home_team_players.clone(),
        };

        let away_team = Team {
            team_id: away_team_id,
            players: self.away_team_players.clone(),
        };

        // return the fully constructed Game
        Some(Game {
            context,
            home_team,
            away_team,
            plays: self.plays.clone(),
        })
    }
}
