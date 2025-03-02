use strum_macros::EnumIter;

pub struct Weather {
    condition: String,
    temperature: u64,
    wind_speed: u64,
}

pub struct Context {
    game_pk: u64,
    date: String,
    venue: String,
    weather: Weather,
}

#[derive(Clone, EnumIter, PartialEq, Eq, Debug)]
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

#[derive(Clone)]
pub struct Player {
    pub position: Position,
    pub name: String,
}

#[derive(Clone)]
pub struct Team {
    team_id: u64,
    players: Vec<Player>,
}

#[derive(Clone)]
pub enum TopBottom {
    Top,
    Bottom,
}

#[derive(Clone)]
pub struct Inning {
    number: u64,
    top_bottom: TopBottom,
}

#[derive(Clone)]
pub enum Base {
    Home,
    First,
    Second,
    Third,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Movement {
    runner: String,
    from: String,
    to: String,
    out: bool,
}

#[derive(Clone)]
pub struct Play {
    inning: Inning,
    play_content: PlayContent,
    movements: Vec<Movement>,
}

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
    pub fn add_play(&mut self, play: Play) -> &mut Self {
        self.plays.push(play);
        self
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
