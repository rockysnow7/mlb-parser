mod game;

use std::collections::HashSet;

use game::{Base, BaseComparison, Game, GameBuilder, Inning, Movement, PlayType, Player, Position, TopBottom};
use once_cell::sync::Lazy;
use pyo3::{prelude::{pyclass, pymethods, PyResult}, exceptions::PyValueError};
use fancy_regex::Regex;
use strum::IntoEnumIterator;

const COMMA_SPACE: &str = r", ";
static CAPTURE_GROUP_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\?P<[^>]+>").unwrap());

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ContextSection {
    Game,
    Date,
    Venue,
    Weather,
}

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TeamSection {
    Team,
    Player,
}

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FieldersSection {
    Tag,
    Name,
    CommaSpace,
}

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MovementsSection {
    Tag,
    Name,
    StartBase,
    Arrow,
    EndBase,
    Out,
    CommaSpace,
    MovementEnd,
}

#[pyclass(eq)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PlaySection {
    GameStart(),
    Inning(),
    Play(),
    Base(),
    Batter(),
    Pitcher(),
    Catcher(),
    Fielders(FieldersSection),
    Runner(),
    ScoringRunner(),
    Movements(MovementsSection),
    PlayEnd(),
    GameEnd(),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameSection {
    Context(ContextSection),
    HomeTeam(TeamSection),
    AwayTeam(TeamSection),
    Plays(PlaySection),
}

const BASE_NAME: &str = r" ?(1|2|3|4|home) ?";
static BASE_NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(format!(
    r"^({})",
    BASE_NAME,
).as_str()).unwrap());
const PLAYER_NAME: &str = r"[a-zA-ZÀ-ÖØ-öø-ÿ.'\- ]+";
static PLAYER_NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(format!(
    r"^{}",
    PLAYER_NAME,
).as_str()).unwrap());
static PLAYER_NAME_BASE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(format!(
    r"^({}?)(?= ?({})\b)",
    PLAYER_NAME,
    BASE_NAME,
).as_str()).unwrap());

static CONTEXT_SECTION_GAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[GAME\] (?P<game_pk>\d{1,6})").unwrap());
static CONTEXT_SECTION_DATE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[DATE\] (?P<date>\d{4}-\d{2}-\d{2})").unwrap());
static CONTEXT_SECTION_VENUE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[VENUE\] (?P<venue>[a-zA-ZÀ-ÖØ-öø-ÿ ]+)").unwrap());
static CONTEXT_SECTION_WEATHER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[WEATHER\] (?P<weather>[a-zA-ZÀ-ÖØ-öø-ÿ ]+) (?P<temperature>\d{1,3}) (?P<wind_speed>\d{1,3})").unwrap());

static TEAM_SECTION_TEAM_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[TEAM\] (?P<team_id>\d{1,3})").unwrap());
static ALL_POSITIONS: Lazy<String> = Lazy::new(|| {
    let mut positions = Vec::new();
    for position in Position::iter() {
        positions.push(position.to_string());
    }

    positions.join("|")
});
static TEAM_SECTION_PLAYER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(format!(
    r"^\[(?P<position>{})\] (?P<player_name>{})",
    ALL_POSITIONS.as_str(),
    PLAYER_NAME,
).as_str()).unwrap());

const PLAY_SECTION_GAME_START: &str = "[GAME_START]";
static PLAY_SECTION_INNING_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[INNING\] (?P<number>\d{1,2}) (?P<top_bottom>top|bottom)").unwrap());
static ALL_PLAY_TYPES: Lazy<String> = Lazy::new(|| {
    let mut play_types = Vec::new();
    for play_type in PlayType::iter() {
        play_types.push(play_type.to_string());
    }
    play_types.sort_by(|a, b| b.len().cmp(&a.len()));

    play_types.join("|")
});
static PLAY_SECTION_PLAY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(format!(
    r"^\[PLAY\] (?P<play_type>{})",
    ALL_PLAY_TYPES.as_str(),
).as_str()).unwrap());
static PLAY_SECTION_BASE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(format!(
    r"^\[BASE\] (?P<base>{})",
    BASE_NAME,
).as_str()).unwrap());
static PLAY_SECTION_BATTER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(format!(
    r"^\[BATTER\] (?P<batter>{})",
    PLAYER_NAME,
).as_str()).unwrap());
static PLAY_SECTION_PITCHER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(format!(
    r"^\[PITCHER\] (?P<pitcher>{})",
    PLAYER_NAME,
).as_str()).unwrap());
static PLAY_SECTION_CATCHER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(format!(
    r"^\[CATCHER\] (?P<catcher>{})",
    PLAYER_NAME,
).as_str()).unwrap());
const PLAY_SECTION_FIELDERS_TAG: &str = "[FIELDERS]";
static PLAY_SECTION_RUNNER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(format!(
    r"^\[RUNNER\] (?P<runner>{})",
    PLAYER_NAME,
).as_str()).unwrap());
static PLAY_SECTION_SCORING_RUNNER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(format!(
    r"^\[SCORING_RUNNER\] (?P<scoring_runner>{})",
    PLAYER_NAME,
).as_str()).unwrap());

const PLAY_SECTION_MOVEMENTS_TAG: &str = "[MOVEMENTS]";
const PLAY_SECTION_ARROW: &str = "->";
const PLAY_SECTION_OUT: &str = "[out]";
const PLAY_SECTION_PLAY_END: &str = ";";
const PLAY_SECTION_GAME_END: &str = "[GAME_END]";

static INITIAL_NEWLINES_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\n+").unwrap());

#[derive(Clone, Debug)]
struct RunnerPositions {
    pub home: Option<String>,
    pub first: Option<String>,
    pub second: Option<String>,
    pub third: Option<String>,
}

impl RunnerPositions {
    pub fn empty() -> Self {
        Self {
            home: None,
            first: None,
            second: None,
            third: None,
        }
    }

    /// Group any chains of movements by the same runner into a single movement.
    fn simplify_movements(&self, movements: &Vec<Movement>) -> Vec<Movement> {
        let runners = HashSet::<String>::from_iter(movements.iter().map(|m| m.runner.clone()));
        // println!("runners: {:#?}", runners);

        let mut simplified_movements = Vec::new();
        for runner in runners {
            // println!("runner: {:#?}", runner);

            let froms = movements.iter().filter(|m| m.runner == runner).map(|m| m.from).collect::<Vec<_>>();
            let tos = movements.iter().filter(|m| m.runner == runner).map(|m| m.to).collect::<Vec<_>>();

            let from = froms.iter().min_by(|a, b| a.compare(b, BaseComparison::From)).unwrap();
            let to = tos.iter().max_by(|a, b| a.compare(b, BaseComparison::To)).unwrap();

            let out = movements.iter().any(|m| m.runner == runner && m.out);

            simplified_movements.push(Movement { runner, from: *from, to: *to, out });
        }

        simplified_movements
    }

    pub fn process_movements(&mut self, movements: &Vec<Movement>, pinch_runners: &Vec<String>) -> Result<(), String> {
        let movements = self.simplify_movements(movements);
        // println!("movements: {:#?}", movements);

        let mut new_runner_positions = self.clone();
        // println!("movements: {:#?}", movements);
        for movement in movements {
            // check the bases are in the correct order
            match (movement.from.clone(), movement.to.clone()) {
                (Base::Third, Base::Second) => return Err("Cannot move runner from third to second".to_string()),
                (Base::Third, Base::First) => return Err("Cannot move runner from third to first".to_string()),
                (Base::Second, Base::First) => return Err("Cannot move runner from second to first".to_string()),
                _ => (),
            }

            // check the runner does exist on the starting base, or that it is a pinch runner
            // println!("movement: {:#?}", movement);
            match movement.from {
                Base::First => match &self.first {
                    Some(runner) => if &movement.runner != runner && !pinch_runners.contains(&movement.runner) {
                        return Err(format!("Runner {} is not on first base and is not a pinch runner", movement.runner));
                    },
                    None => return Err("No runner is on first base".to_string()),
                },
                Base::Second => match &self.second {
                    Some(runner) => if &movement.runner != runner && !pinch_runners.contains(&movement.runner) {
                        return Err(format!("Runner {} is not on second base and is not a pinch runner", movement.runner));
                    },
                    None => return Err("No runner is on second base".to_string()),
                },
                Base::Third => match &self.third {
                    Some(runner) => if &movement.runner != runner && !pinch_runners.contains(&movement.runner) {
                        return Err(format!("Runner {} is not on third base and is not a pinch runner", movement.runner));
                    },
                    None => return Err("No runner is on third base".to_string()),
                },
                Base::Home => (),
            }

            // if the runner is not out, move the runner to the new base
            if !movement.out {
                match movement.to {
                    Base::First => new_runner_positions.first = Some(movement.runner.clone()),
                    Base::Second => new_runner_positions.second = Some(movement.runner.clone()),
                    Base::Third => new_runner_positions.third = Some(movement.runner.clone()),
                    Base::Home => new_runner_positions.home = Some(movement.runner.clone()),
                }
            }
        }

        // update the runner positions
        *self = new_runner_positions;
        // println!("runner positions: {:#?}", self);

        Ok(())
    }
}

struct LiveGameState {
    pub runner_positions: RunnerPositions,
    pub inning: Inning,
    pub home_team_score: u64,
    pub away_team_score: u64,
}

impl LiveGameState {
    pub fn new() -> Self {
        Self {
            runner_positions: RunnerPositions::empty(),
            inning: Inning { number: 1, top_bottom: TopBottom::Top },
            home_team_score: 0,
            away_team_score: 0,
        }
    }
}

#[pyclass]
pub struct Parser {
    input_buffer: String,
    possible_sections: Vec<GameSection>,
    game_builder: GameBuilder,
    #[pyo3(get)]
    finished: bool,
    print_debug: bool,
    live_game_state: LiveGameState,
    pinch_runners: Vec<String>,
}

impl Parser {
    fn print_debug_message(&self) {
        println!("possible_sections: {:#?}", self.possible_sections);
        println!("input_buffer.take(100): {:?}", self.input_buffer.chars().take(100).collect::<String>());
        // println!("movement_builder: {:#?}\n", self.game_builder.play_builder.movement_builder);
    }

    fn consume_input(&mut self, index: usize) {
        self.input_buffer = self.input_buffer
            .split_off(index)
            .trim_start()
            .to_string();
    }

    fn parse_context_section(&mut self, context_section: ContextSection) -> PyResult<bool> {
        match context_section {
            ContextSection::Game => {
                let captures = CONTEXT_SECTION_GAME_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let game_pk_match = captures.name("game_pk").unwrap();
                    let game_pk = game_pk_match.as_str().parse::<u64>().unwrap();
                    self.game_builder.set_game_pk(game_pk);

                    if game_pk_match.end() == self.input_buffer.len() {
                        return Ok(false);
                    }

                    self.consume_input(game_pk_match.end());
                    self.possible_sections = vec![GameSection::Context(ContextSection::Date)];

                    return Ok(true);
                }
            },
            ContextSection::Date => {
                let captures = CONTEXT_SECTION_DATE_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let date_match = captures.name("date").unwrap();
                    let date = date_match.as_str().to_string();
                    self.game_builder.set_date(date);

                    if date_match.end() == self.input_buffer.len() {
                        return Ok(false);
                    }

                    self.consume_input(date_match.end());
                    self.possible_sections = vec![GameSection::Context(ContextSection::Venue)];

                    return Ok(true);
                }
            },
            ContextSection::Venue => {
                let captures = CONTEXT_SECTION_VENUE_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let venue_match = captures.name("venue").unwrap();
                    let venue = venue_match.as_str().trim().to_string();
                    self.game_builder.set_venue(venue);

                    if venue_match.end() == self.input_buffer.len() {
                        return Ok(false);
                    }

                    self.consume_input(venue_match.end());
                    self.possible_sections = vec![GameSection::Context(ContextSection::Weather)];

                    return Ok(true);
                }
            },
            ContextSection::Weather => {
                let captures = CONTEXT_SECTION_WEATHER_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let weather_match = captures.name("weather").unwrap();
                    let weather = weather_match.as_str().to_string();

                    let temperature_match = captures.name("temperature").unwrap();
                    let temperature = temperature_match.as_str().parse::<u64>().unwrap();

                    let wind_speed_match = captures.name("wind_speed").unwrap();
                    let wind_speed = wind_speed_match.as_str().parse::<u64>().unwrap();

                    self.game_builder.set_weather(weather, temperature, wind_speed);

                    if wind_speed_match.end() == self.input_buffer.len() {
                        return Ok(false);
                    }

                    self.consume_input(wind_speed_match.end());
                    self.possible_sections = vec![GameSection::HomeTeam(TeamSection::Team)];

                    return Ok(true);
                }
            },
        }

        Ok(false)
    }

    fn parse_team_section(&mut self, team_section: TeamSection, home_team: bool) -> PyResult<bool> {
        match team_section {
            TeamSection::Team => {
                let captures = TEAM_SECTION_TEAM_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let team_id_match = captures.name("team_id").unwrap();
                    let team_id = team_id_match.as_str().parse::<u64>().unwrap();

                    if home_team {
                        self.game_builder.set_home_team_id(team_id);
                    } else {
                        self.game_builder.set_away_team_id(team_id);
                    }

                    if team_id_match.end() == self.input_buffer.len() {
                        return Ok(false);
                    }

                    self.consume_input(team_id_match.end());

                    if home_team {
                        self.possible_sections = vec![GameSection::HomeTeam(TeamSection::Player)];
                    } else {
                        self.possible_sections = vec![GameSection::AwayTeam(TeamSection::Player)];
                    }

                    return Ok(true);
                }
            },
            TeamSection::Player => {
                let captures = TEAM_SECTION_PLAYER_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let position_match = captures.name("position").unwrap();
                    let position = position_match.as_str().parse::<Position>().unwrap();

                    let player_name_match = captures.name("player_name").unwrap();
                    let player_name = player_name_match.as_str().trim().to_string();

                    let player = Player {
                        position,
                        name: player_name.clone(),
                    };

                    if position == Position::PinchRunner {
                        self.pinch_runners.push(player_name);
                    }

                    if player_name_match.end() == self.input_buffer.len() {
                        return Ok(false);
                    }

                    self.consume_input(player_name_match.end());

                    if home_team {
                        self.game_builder.add_home_team_player(player);
                        self.possible_sections = vec![
                            GameSection::HomeTeam(TeamSection::Player),
                            GameSection::AwayTeam(TeamSection::Team),
                        ];
                    } else {
                        self.game_builder.add_away_team_player(player);
                        self.possible_sections = vec![
                            GameSection::AwayTeam(TeamSection::Player),
                            GameSection::Plays(PlaySection::GameStart()),
                        ];
                    }

                    return Ok(true);
                }
            },
        }

        Ok(false)
    }

    fn parse_play_section(&mut self, play_section: PlaySection) -> PyResult<bool> {
        match play_section {
            PlaySection::GameStart() => {
                if self.input_buffer.starts_with(PLAY_SECTION_GAME_START) {
                    self.consume_input(PLAY_SECTION_GAME_START.len());
                    self.possible_sections = vec![GameSection::Plays(PlaySection::Inning())];

                    return Ok(true);
                }
            },
            PlaySection::Inning() => {
                let captures = PLAY_SECTION_INNING_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let number_match = captures.name("number").unwrap();
                    let number = number_match.as_str().parse::<u64>().unwrap();

                    let top_bottom_match = captures.name("top_bottom").unwrap();
                    let top_bottom = top_bottom_match.as_str().parse::<TopBottom>().unwrap();

                    let inning = Inning {
                        number,
                        top_bottom,
                    };

                    self.game_builder.play_builder.set_inning(inning);

                    if top_bottom_match.end() == self.input_buffer.len() {
                        return Ok(false);
                    }

                    if self.live_game_state.inning.top_bottom != top_bottom {
                        self.live_game_state.runner_positions = RunnerPositions::empty();
                    }
                    self.live_game_state.inning = inning;

                    self.consume_input(top_bottom_match.end());
                    self.possible_sections = vec![GameSection::Plays(PlaySection::Play())];

                    return Ok(true);
                }
            },
            PlaySection::Play() => {
                let captures = PLAY_SECTION_PLAY_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let play_type_match = captures.name("play_type").unwrap();
                    let play_type = play_type_match.as_str().parse::<PlayType>().unwrap();

                    self.game_builder.play_builder.set_play_type(play_type);

                    if play_type_match.end() == self.input_buffer.len() {
                        return Ok(false);
                    }

                    self.consume_input(play_type_match.end());

                    if play_type == PlayType::GameAdvisory {
                        self.game_builder.build_play();
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Inning()),
                            GameSection::Plays(PlaySection::GameEnd()),
                        ];
                    } else if play_type.requires_base() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Base()),
                        ];
                    } else if play_type.requires_batter() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Batter()),
                        ];
                    } else if play_type.requires_pitcher() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Pitcher()),
                        ];
                    } else if play_type.requires_catcher() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Catcher()),
                        ];
                    } else if play_type.requires_fielders() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Fielders(FieldersSection::Tag)),
                        ];
                    } else if play_type.requires_runner() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Runner()),
                        ];
                    } else if play_type.requires_scoring_runner() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::ScoringRunner()),
                        ];
                    } else {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Movements(MovementsSection::Tag)),
                        ];
                    }

                    return Ok(true);
                }
            },
            PlaySection::Base() => {
                let captures = PLAY_SECTION_BASE_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let base_match = captures.name("base").unwrap();
                    let base = base_match.as_str().trim().parse::<Base>().unwrap();

                    self.game_builder.play_builder.set_base(base);

                    if base_match.end() == self.input_buffer.len() {
                        return Ok(false);
                    }

                    self.consume_input(base_match.end());

                    let play_type = self.game_builder.play_builder.play_type.unwrap();
                    if play_type.requires_batter() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Batter()),
                        ];
                    } else if play_type.requires_pitcher() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Pitcher()),
                        ];
                    } else if play_type.requires_catcher() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Catcher()),
                        ];
                    } else if play_type.requires_runner() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Runner()),
                        ];
                    } else if play_type.requires_fielders() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Fielders(FieldersSection::Tag)),
                        ];
                    } else if play_type.requires_scoring_runner() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::ScoringRunner()),
                        ];
                    } else {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Movements(MovementsSection::Tag)),
                        ];
                    }

                    return Ok(true);
                }
            },
            PlaySection::Batter() => {
                let captures = PLAY_SECTION_BATTER_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let batter_match = captures.name("batter").unwrap();
                    let batter = batter_match.as_str().trim().to_string();

                    self.game_builder.play_builder.set_batter(batter);

                    if batter_match.end() == self.input_buffer.len() {
                        return Ok(false);
                    }

                    self.consume_input(batter_match.end());

                    let play_type = self.game_builder.play_builder.play_type.unwrap();
                    if play_type.requires_pitcher() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Pitcher()),
                        ];
                    } else if play_type.requires_catcher() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Catcher()),
                        ];
                    } else if play_type.requires_fielders() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Fielders(FieldersSection::Tag)),
                        ];
                    } else if play_type.requires_runner() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Runner()),
                        ];
                    } else if play_type.requires_scoring_runner() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::ScoringRunner()),
                        ];
                    } else {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Movements(MovementsSection::Tag)),
                        ];
                    }

                    return Ok(true);
                }
            },
            PlaySection::Pitcher() => {
                let captures = PLAY_SECTION_PITCHER_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let pitcher_match = captures.name("pitcher").unwrap();
                    let pitcher = pitcher_match.as_str().trim().to_string();

                    self.game_builder.play_builder.set_pitcher(pitcher);

                    if pitcher_match.end() == self.input_buffer.len() {
                        return Ok(false);
                    }

                    self.consume_input(pitcher_match.end());

                    let play_type = self.game_builder.play_builder.play_type.unwrap();
                    if play_type.requires_catcher() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Catcher()),
                        ];
                    } else if play_type.requires_fielders() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Fielders(FieldersSection::Tag)),
                        ];
                    } else if play_type.requires_runner() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Runner()),
                        ];
                    } else if play_type.requires_scoring_runner() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::ScoringRunner()),
                        ];
                    } else {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Movements(MovementsSection::Tag)),
                        ];
                    }

                    return Ok(true);
                }
            },
            PlaySection::Catcher() => {
                let captures = PLAY_SECTION_CATCHER_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let catcher_match = captures.name("catcher").unwrap();
                    let catcher = catcher_match.as_str().trim().to_string();

                    self.game_builder.play_builder.set_catcher(catcher);

                    if catcher_match.end() == self.input_buffer.len() {
                        return Ok(false);
                    }

                    self.consume_input(catcher_match.end());

                    let play_type = self.game_builder.play_builder.play_type.unwrap();
                    if play_type.requires_fielders() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Fielders(FieldersSection::Tag)),
                        ];
                    } else if play_type.requires_runner() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Runner()),
                        ];
                    } else if play_type.requires_scoring_runner() {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::ScoringRunner()),
                        ];
                    } else {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Movements(MovementsSection::Tag)),
                        ];
                    }

                    return Ok(true);
                }
            },
            PlaySection::Fielders(fielders_section) => {
                match fielders_section {
                    FieldersSection::Tag => {
                        if self.input_buffer.starts_with(PLAY_SECTION_FIELDERS_TAG) {
                            self.consume_input(PLAY_SECTION_FIELDERS_TAG.len());
                            self.possible_sections = vec![GameSection::Plays(PlaySection::Fielders(FieldersSection::Name))];

                            return Ok(true);
                        }
                    },
                    FieldersSection::Name => {
                        let mut matches = PLAYER_NAME_REGEX.find_iter(&self.input_buffer);
                        let player_name_match = matches.next();
                        if let Some(Ok(player_name_match)) = player_name_match {
                            let player_name = player_name_match.as_str().trim().to_string();

                            if player_name_match.end() == self.input_buffer.len() {
                                return Ok(false);
                            }

                            self.game_builder.play_builder.add_fielder(player_name);
                            self.consume_input(player_name_match.end());

                            self.possible_sections = vec![
                                GameSection::Plays(PlaySection::Fielders(FieldersSection::CommaSpace)),
                            ];
                            let play_type = self.game_builder.play_builder.play_type.unwrap();
                            if play_type.requires_scoring_runner() {
                                self.possible_sections.push(GameSection::Plays(PlaySection::ScoringRunner()));
                            } else {
                                self.possible_sections.push(GameSection::Plays(PlaySection::Movements(MovementsSection::Tag)));
                            }

                            return Ok(true);
                        }
                    },
                    FieldersSection::CommaSpace => {
                        if self.input_buffer.starts_with(COMMA_SPACE) {
                            self.consume_input(COMMA_SPACE.len());
                            self.possible_sections = vec![GameSection::Plays(PlaySection::Fielders(FieldersSection::Name))];

                            return Ok(true);
                        }
                    },
                }
            },
            PlaySection::Runner() => {
                let captures = PLAY_SECTION_RUNNER_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let runner_match = captures.name("runner").unwrap();
                    let runner = runner_match.as_str().trim().to_string();

                    self.game_builder.play_builder.set_runner(runner);

                    if runner_match.end() == self.input_buffer.len() {
                        return Ok(false);
                    }

                    self.consume_input(runner_match.end());

                    let play_type = self.game_builder.play_builder.play_type.unwrap();
                    if play_type.requires_scoring_runner() {
                        self.possible_sections = vec![GameSection::Plays(PlaySection::ScoringRunner())];
                    } else if play_type.requires_fielders() {
                        self.possible_sections = vec![GameSection::Plays(PlaySection::Fielders(FieldersSection::Tag))];
                    } else {
                        self.possible_sections = vec![GameSection::Plays(PlaySection::Movements(MovementsSection::Tag))];
                    }

                    return Ok(true);
                }
            },
            PlaySection::ScoringRunner() => {
                let captures = PLAY_SECTION_SCORING_RUNNER_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let scoring_runner_match = captures.name("scoring_runner").unwrap();
                    let scoring_runner = scoring_runner_match.as_str().trim().to_string();

                    self.game_builder.play_builder.set_scoring_runner(scoring_runner);

                    if scoring_runner_match.end() == self.input_buffer.len() {
                        return Ok(false);
                    }

                    self.consume_input(scoring_runner_match.end());
                    self.possible_sections = vec![GameSection::Plays(PlaySection::Movements(MovementsSection::Tag))];

                    return Ok(true);
                }
            },
            PlaySection::Movements(movements_section) => {
                match movements_section {
                    MovementsSection::Tag => {
                        if self.input_buffer.starts_with(PLAY_SECTION_MOVEMENTS_TAG) {
                            self.consume_input(PLAY_SECTION_MOVEMENTS_TAG.len());
                            self.possible_sections = vec![GameSection::Plays(PlaySection::Movements(MovementsSection::Name))];

                            return Ok(true);
                        }
                    },
                    MovementsSection::Name => {
                        let mut matches = PLAYER_NAME_BASE_REGEX.find_iter(&self.input_buffer);
                        let player_name_match = matches.next();
                        if let Some(Ok(player_name_match)) = player_name_match {
                            let mut player_name = player_name_match.as_str().trim().to_string();

                            if player_name_match.end() == self.input_buffer.len() {
                                return Ok(false);
                            }

                            player_name = player_name.trim().to_string();
                            self.game_builder.play_builder.movement_builder.set_runner(player_name);

                            self.consume_input(player_name_match.end());
                            self.possible_sections = vec![GameSection::Plays(PlaySection::Movements(MovementsSection::StartBase))];

                            return Ok(true);
                        }
                    },
                    MovementsSection::StartBase => {
                        let mut matches = BASE_NAME_REGEX.find_iter(&self.input_buffer);
                        let base_match = matches.next();
                        if let Some(Ok(base_match)) = base_match {
                            let base = base_match.as_str().trim().parse::<Base>().unwrap();

                            self.game_builder.play_builder.movement_builder.set_from(base);

                            if base_match.end() == self.input_buffer.len() {
                                return Ok(false);
                            }

                            self.consume_input(base_match.end());
                            self.possible_sections = vec![GameSection::Plays(PlaySection::Movements(MovementsSection::Arrow))];

                            return Ok(true);
                        }
                    },
                    MovementsSection::Arrow => {
                        if self.input_buffer.starts_with(PLAY_SECTION_ARROW) {
                            self.consume_input(PLAY_SECTION_ARROW.len());
                            self.possible_sections = vec![GameSection::Plays(PlaySection::Movements(MovementsSection::EndBase))];

                            return Ok(true);
                        }
                    },
                    MovementsSection::EndBase => {
                        let mut matches = BASE_NAME_REGEX.find_iter(&self.input_buffer);
                        let base_match = matches.next();
                        if let Some(Ok(base_match)) = base_match {
                            let base = base_match.as_str().trim().parse::<Base>().unwrap();

                            self.game_builder.play_builder.movement_builder.set_to(base);

                            if base_match.end() == self.input_buffer.len() {
                                return Ok(false);
                            }

                            self.consume_input(base_match.end());
                            self.possible_sections = vec![
                                GameSection::Plays(PlaySection::Movements(MovementsSection::Out)),
                                GameSection::Plays(PlaySection::Movements(MovementsSection::MovementEnd)),
                            ];

                            return Ok(true);
                        }
                    },
                    MovementsSection::Out => {
                        if self.input_buffer.starts_with(PLAY_SECTION_OUT) {
                            self.game_builder.play_builder.movement_builder.set_out();

                            if self.input_buffer.len() == PLAY_SECTION_OUT.len() {
                                return Ok(false);
                            }

                            self.consume_input(PLAY_SECTION_OUT.len());

                            self.possible_sections = vec![
                                GameSection::Plays(PlaySection::Movements(MovementsSection::MovementEnd)),
                            ];

                            return Ok(true);
                        }
                    },
                    MovementsSection::CommaSpace => {
                        if self.input_buffer.starts_with(COMMA_SPACE) {
                            let _ = self.game_builder.play_builder.build_movement();

                            self.consume_input(COMMA_SPACE.len());
                            self.possible_sections = vec![GameSection::Plays(PlaySection::Movements(MovementsSection::Name))];

                            return Ok(true);
                        }
                    },
                    MovementsSection::MovementEnd => {
                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Movements(MovementsSection::Out)),
                            GameSection::Plays(PlaySection::Movements(MovementsSection::CommaSpace)),
                            GameSection::Plays(PlaySection::PlayEnd()),
                        ];

                        return Ok(true);
                    },
                }
            },
            PlaySection::PlayEnd() => {
                if self.input_buffer.starts_with(PLAY_SECTION_PLAY_END) {
                    let _ = self.game_builder.play_builder.build_movement();

                    self.consume_input(PLAY_SECTION_PLAY_END.len());

                    self.game_builder.build_play();

                    let movements = &self.game_builder.plays.last().unwrap().movements;
                    if let Err(e) = self.live_game_state.runner_positions.process_movements(movements, &self.pinch_runners) {
                        // println!("error while processing movements");
                        return Err(PyValueError::new_err(format!(
                            "Inning {}: {}",
                            &self.game_builder.plays.last().unwrap().inning.to_string(),
                            e,
                        )));
                    } else {
                        // println!("no error while processing movements.");
                    }

                    self.possible_sections = vec![
                        GameSection::Plays(PlaySection::Inning()),
                        GameSection::Plays(PlaySection::GameEnd()),
                    ];

                    return Ok(true);
                }

                return Ok(false);
            },
            PlaySection::GameEnd() => {
                if self.input_buffer.starts_with(PLAY_SECTION_GAME_END) {
                    self.consume_input(PLAY_SECTION_GAME_END.len());
                    self.finished = true;

                    return Ok(true);
                }

                return Ok(false);
            },
        }

        Ok(false)
    }

    fn parse_input_buffer(&mut self) -> PyResult<bool> {
        for section in self.possible_sections.clone() {
            let success = match section {
                GameSection::Context(context_section) => {
                    if self.print_debug {
                        self.print_debug_message();
                    }

                    self.parse_context_section(context_section)
                },
                GameSection::HomeTeam(team_section) => {
                    if self.print_debug {
                        self.print_debug_message();
                    }

                    self.parse_team_section(team_section, true)
                },
                GameSection::AwayTeam(team_section) => {
                    if self.print_debug {
                        self.print_debug_message();
                    }

                    self.parse_team_section(team_section, false)
                },
                GameSection::Plays(play_section) => {
                    if self.print_debug {
                        self.print_debug_message();
                    }

                    self.parse_play_section(play_section)
                },
            }?;

            if success {
                return Ok(success);
            }
        }

        Ok(false)
    }

    /// Return a regex that matches the inner part of a play of a given type.
    fn inner_pattern_from_play_type(&self, play_type: &PlayType) -> String {
        let mut s = format!(r"\[PLAY\] {} ", play_type.to_string());

        if play_type.requires_base() {
            let base = CAPTURE_GROUP_REGEX.replace_all(PLAY_SECTION_BASE_REGEX.as_str(), "");
            s.push_str(&base);
            s.push_str(" ");
        }
        if play_type.requires_batter() {
            let batter = CAPTURE_GROUP_REGEX.replace_all(PLAY_SECTION_BATTER_REGEX.as_str(), "");
            s.push_str(&batter);
            s.push_str(" ");
        }
        if play_type.requires_pitcher() {
            let pitcher = CAPTURE_GROUP_REGEX.replace_all(PLAY_SECTION_PITCHER_REGEX.as_str(), "");
            s.push_str(&pitcher);
            s.push_str(" ");
        }
        if play_type.requires_catcher() {
            let catcher = CAPTURE_GROUP_REGEX.replace_all(PLAY_SECTION_CATCHER_REGEX.as_str(), "");
            s.push_str(&catcher);
            s.push_str(" ");
        }
        if play_type.requires_fielders() {
            let fielders = format!(
                "{tag} {name}(, {name})*",
                tag=PLAY_SECTION_FIELDERS_TAG.replace("[", r"\[").replace("]", r"\]"),
                name=PLAYER_NAME,
            );

            s.push_str(&fielders);
            s.push_str(" ");
        }
        if play_type.requires_runner() {
            let runner = CAPTURE_GROUP_REGEX.replace_all(PLAY_SECTION_RUNNER_REGEX.as_str(), "");
            s.push_str(&runner);
            s.push_str(" ");
        }
        if play_type.requires_scoring_runner() {
            let scoring_runner = CAPTURE_GROUP_REGEX.replace_all(PLAY_SECTION_SCORING_RUNNER_REGEX.as_str(), "");
            s.push_str(&scoring_runner);
            s.push_str(" ");
        }

        s.trim().replace("^", "")
    }

    /// Return a regex that matches the movements part of a play.
    fn movements_regex(&self) -> String {
        let mut s = PLAY_SECTION_MOVEMENTS_TAG.replace("[", r"\[").replace("]", r"\]");
        s.push_str(" ");

        let pinch_runners = self.pinch_runners.join("|");

        let mut valid_movement_patterns = Vec::new();
        let home_or_pinch_runner = if pinch_runners.is_empty() {
            PLAYER_NAME.to_string()
        } else {
            format!(r"({}|{})", PLAYER_NAME, pinch_runners)
        };
        let home_to_any = format!(r"{home_or_pinch_runner} home -> (1|2|3|4|home)( \[out\])?");
        valid_movement_patterns.push(home_to_any);

        if let Some(first) = &self.live_game_state.runner_positions.first {
            let first_or_pinch_runner = if pinch_runners.is_empty() {
                first.to_string()
            } else {
                format!(r"({}|{})", first, pinch_runners)
            };
            let first_to_any = format!(r"{first_or_pinch_runner} 1 -> (2|3|4|home)( \[out\])?");
            valid_movement_patterns.push(first_to_any);
        }

        if let Some(second) = &self.live_game_state.runner_positions.second {
            let second_or_pinch_runner = if pinch_runners.is_empty() {
                second.to_string()
            } else {
                format!(r"({}|{})", second, pinch_runners)
            };
            let second_to_any = format!(r"{second_or_pinch_runner} 2 -> (3|4|home)( \[out\])?");
            valid_movement_patterns.push(second_to_any);
        }

        if let Some(third) = &self.live_game_state.runner_positions.third {
            let third_or_pinch_runner = if pinch_runners.is_empty() {
                third.to_string()
            } else {
                format!(r"({}|{})", third, pinch_runners)
            };
            let third_to_any = format!(r"{third_or_pinch_runner} 3 -> (4|home)( \[out\])?");
            valid_movement_patterns.push(third_to_any);
        }

        let joined = valid_movement_patterns.iter()
            .map(|s| format!("({})", s))
            .collect::<Vec<_>>()
            .join("|");
        let many = format!(r"{joined}(, {joined})*");
        s.push_str(&many);

        s
    }

    /// Return a regex that matches a single play.
    pub fn play_regex(&self) -> String {
        let inning = CAPTURE_GROUP_REGEX.replace_all(PLAY_SECTION_INNING_REGEX.as_str(), "").replace("^", "");
        let all_plays = PlayType::iter().map(|play_type| self.inner_pattern_from_play_type(&play_type)).collect::<Vec<_>>();
        let inner = all_plays.iter().map(|s| format!("({})", s)).collect::<Vec<_>>().join("|");
        let movements = self.movements_regex();

        format!(
            "{} ({}) {}{}",
            inning,
            inner,
            movements,
            PLAY_SECTION_PLAY_END,
        )
    }
}

#[pymethods]
impl Parser {
    #[staticmethod]
    fn new(print_debug: bool) -> Self {
        Self {
            input_buffer: String::new(),
            possible_sections: vec![GameSection::Context(ContextSection::Game)],
            game_builder: GameBuilder::new(),
            finished: false,
            print_debug,
            live_game_state: LiveGameState::new(),
            pinch_runners: Vec::new(),
        }
    }

    /// Stream-parse a game and return the set of valid next characters.
    pub fn parse_input(&mut self, input: &str) -> PyResult<()> {
        let input = INITIAL_NEWLINES_REGEX.replace(input, "");
        self.input_buffer.push_str(&input);

        loop {
            if self.finished {
                return Ok(());
            }

            let success = self.parse_input_buffer()?;

            if !success {
                return Ok(());
            }
        }
    }

    /// Return the completed game if the parser is finished.
    pub fn complete(&self) -> Option<Game> {
        if self.finished {
            self.game_builder.build()
        } else {
            None
        }
    }

    /// Return a regex that matches a full valid game, taking into account the current game state.
    pub fn valid_regex(&self) -> String {
        let game = CAPTURE_GROUP_REGEX.replace_all(CONTEXT_SECTION_GAME_REGEX.as_str(), "").replace("^", "");
        let date = CAPTURE_GROUP_REGEX.replace_all(CONTEXT_SECTION_DATE_REGEX.as_str(), "").replace("^", "");
        let venue = CAPTURE_GROUP_REGEX.replace_all(CONTEXT_SECTION_VENUE_REGEX.as_str(), "").replace("^", "");
        let weather = CAPTURE_GROUP_REGEX.replace_all(CONTEXT_SECTION_WEATHER_REGEX.as_str(), "").replace("^", "");
        let context_section_regex = format!(
            "{} {} {} {}",
            game,
            date,
            venue,
            weather,
        );

        let team = CAPTURE_GROUP_REGEX.replace_all(TEAM_SECTION_TEAM_REGEX.as_str(), "").replace("^", "");
        let player = CAPTURE_GROUP_REGEX.replace_all(TEAM_SECTION_PLAYER_REGEX.as_str(), "").replace("^", "");
        let team_section_regex = format!(
            "{}\n({})(\n{})*",
            team,
            player,
            player,
        );

        let game_start = PLAY_SECTION_GAME_START.replace("[", r"\[").replace("]", r"\]");
        let play_end = PLAY_SECTION_PLAY_END.replace("[", r"\[").replace("]", r"\]");
        let play_section_regex = format!(
            "{}\n({}\n)+{}",
            game_start,
            self.play_regex(),
            play_end,
        );

        format!(
            "{}\n\n{}\n\n{}\n\n{}",
            context_section_regex,
            team_section_regex,
            team_section_regex,
            play_section_regex,
        ).replace("^", "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parsing_tests {
        use super::*;

        #[test]
        fn parse_game_pk() {
            let mut parser = Parser::new(false);
            let input = "[GAME] 766493";
            let _ = parser.parse_input(input);

            if let Some(game_pk) = parser.game_builder.game_pk {
                assert_eq!(game_pk, 766493);
            } else {
                panic!("game_pk is None");
            }
        }

        #[test]
        fn parse_date() {
            let mut parser = Parser::new(false);
            let input = "[GAME] 766493 [DATE] 2024-03-24";

            let _ = parser.parse_input(input);

            if let Some(date) = parser.game_builder.date {
                assert_eq!(date, "2024-03-24");
            } else {
                panic!("date is None");
            }
        }

        #[test]
        fn parse_partial_input_is_ok() {
            let mut parser = Parser::new(false);
            let input = "[GAM";
            let result = parser.parse_input(input);

            assert!(result.is_ok());
            assert_eq!(parser.possible_sections, vec![GameSection::Context(ContextSection::Game)]);

            let input = "E] 766493";
            let _ = parser.parse_input(input);

            if let Some(game_pk) = parser.game_builder.game_pk {
                assert_eq!(game_pk, 766493);
            } else {
                panic!("game_pk is None");
            }
        }

        #[test]
        fn parse_entire_context_section() {
            let mut parser = Parser::new(false);
            let input = "[GAME] 766493 [DATE] 2024-03-24 [VENUE] Estadio Alfredo Harp Helu [WEATHER] Sunny 85 9";

            let _ = parser.parse_input(input);

            if let Some(game_pk) = parser.game_builder.game_pk {
                assert_eq!(game_pk, 766493);
            } else {
                panic!("game_pk is None");
            }

            if let Some(date) = parser.game_builder.date {
                assert_eq!(date, "2024-03-24");
            } else {
                panic!("date is None");
            }

            if let Some(venue) = parser.game_builder.venue {
                assert_eq!(venue, "Estadio Alfredo Harp Helu");
            } else {
                panic!("venue is None");
            }

            if let Some(weather_condition) = parser.game_builder.weather_condition {
                assert_eq!(weather_condition, "Sunny");
            } else {
                panic!("weather_condition is None");
            }

            if let Some(temperature) = parser.game_builder.weather_temperature {
                assert_eq!(temperature, 85);
            } else {
                panic!("temperature is None");
            }

            if let Some(wind_speed) = parser.game_builder.weather_wind_speed {
                assert_eq!(wind_speed, 9);
            } else {
                panic!("wind_speed is None");
            }
        }

        #[test]
        fn parse_home_team_section() {
            let mut parser = Parser::new(false);
            let input = "[GAME] 0 [DATE] 0000-00-00 [VENUE] venue [WEATHER] weather 0 0\n\n[TEAM] 20\n[SECOND_BASE] Robinson Canó\n[PITCHER] Arturo Lopez [";

            let _ = parser.parse_input(input);

            if let Some(home_team_id) = parser.game_builder.home_team_id {
                assert_eq!(home_team_id, 20);
            } else {
                panic!("home_team_id is None");
            }

            assert!(!parser.game_builder.home_team_players.is_empty());

            assert_eq!(parser.game_builder.home_team_players[0].position, Position::SecondBase);
            assert_eq!(parser.game_builder.home_team_players[0].name, "Robinson Canó");

            assert_eq!(parser.game_builder.home_team_players[1].position, Position::Pitcher);
            assert_eq!(parser.game_builder.home_team_players[1].name, "Arturo Lopez");
        }

        #[test]
        fn parse_away_team_section() {
            let mut parser = Parser::new(false);
            let input = "[GAME] 0 [DATE] 0000-00-00 [VENUE] venue [WEATHER] weather 0 0\n\n[TEAM] 20\n[SECOND_BASE] Robinson Canó\n[PITCHER] Arturo Lopez [TEAM] 147 [THIRD_BASE] DJ LeMahieu [FIRST_BASE] Anthony Rizzo [";

            let _ = parser.parse_input(input);

            if let Some(away_team_id) = parser.game_builder.away_team_id {
                assert_eq!(away_team_id, 147);
            } else {
                panic!("away_team_id is None");
            }

            assert!(!parser.game_builder.away_team_players.is_empty());

            assert_eq!(parser.game_builder.away_team_players[0].position, Position::ThirdBase);
            assert_eq!(parser.game_builder.away_team_players[0].name, "DJ LeMahieu");

            assert_eq!(parser.game_builder.away_team_players[1].position, Position::FirstBase);
            assert_eq!(parser.game_builder.away_team_players[1].name, "Anthony Rizzo");
        }

        #[test]
        fn parse_simple_play() {
            use game::{PlayContent, Movement};

            let mut parser = Parser::new(false);
            let input = "[GAME] 766493 [DATE] 2024-03-24 [VENUE] Estadio Alfredo Harp Helu [WEATHER] Sunny 85 9 [TEAM] 20 [SECOND_BASE] Robinson Canó [TEAM] 147 [THIRD_BASE] DJ LeMahieu [GAME_START] [INNING] 1 top [PLAY] Lineout [BATTER] Anthony Volpe [PITCHER] Trevor Bauer [FIELDERS] Aristides Aquino [MOVEMENTS] Anthony Volpe home -> home [out];";

            let _ = parser.parse_input(input);

            if let Some(play) = parser.game_builder.plays.iter().next() {
                assert!(play.inning == Inning { number: 1, top_bottom: TopBottom::Top });
                assert!(play.play_content == PlayContent::Lineout {
                    batter: "Anthony Volpe".to_string(),
                    pitcher: "Trevor Bauer".to_string(),
                    fielders: vec!["Aristides Aquino".to_string()],
                });
                assert!(play.movements == vec![
                    Movement {
                        runner: "Anthony Volpe".to_string(),
                        from: Base::Home,
                        to: Base::Home,
                        out: true,
                    },
                ]);
            } else {
                panic!("play is None");
            }
        }

        #[test]
        fn parse_complex_play() {
            use game::{PlayContent, Movement};
            let mut parser = Parser::new(false);
            let input = "[GAME] 766493 [DATE] 2024-03-24 [VENUE] Estadio Alfredo Harp Helu [WEATHER] Sunny 85 9 [TEAM] 20 [SECOND_BASE] Robinson Canó [TEAM] 147 [THIRD_BASE] DJ LeMahieu [GAME_START] [INNING] 1 top [PLAY] Groundout [BATTER] Juan Carlos Gamboa [PITCHER] Tanner Tully [FIELDERS] Tanner Tully, Trevor Bauer [MOVEMENTS] Juan Carlos Gamboa home -> home [out], Xavier Fernández home -> 2;";

            let _ = parser.parse_input(input);

            if let Some(play) = parser.game_builder.plays.iter().next() {
                assert!(play.inning == Inning { number: 1, top_bottom: TopBottom::Top });
                assert!(play.play_content == PlayContent::Groundout {
                    batter: "Juan Carlos Gamboa".to_string(),
                    pitcher: "Tanner Tully".to_string(),
                    fielders: vec!["Tanner Tully".to_string(), "Trevor Bauer".to_string()],
                });
                assert!(play.movements == vec![
                    Movement {
                        runner: "Juan Carlos Gamboa".to_string(),
                        from: Base::Home,
                        to: Base::Home,
                        out: true,
                    },
                    Movement {
                        runner: "Xavier Fernández".to_string(),
                        from: Base::Home,
                        to: Base::Second,
                        out: false,
                    },
                ]);
            } else {
                panic!("play is None");
            }
        }

        #[test]
        fn parse_very_broken_up_input() {
            use game::{PlayContent, Movement};

            let mut parser = Parser::new(false);

            let _ = parser.parse_input("[GAM");
            let _ = parser.parse_input("E] 766");
            let _ = parser.parse_input("493 [DATE] 2024-");
            let _ = parser.parse_input("03-2");
            let _ = parser.parse_input("4 [VENUE] E");
            let _ = parser.parse_input("stadio Alfred");
            let _ = parser.parse_input("o Harp Helu [WEATHER] Sun");
            let _ = parser.parse_input("ny 8");
            let _ = parser.parse_input("5 9");
            let _ = parser.parse_input("1");

            let _ = parser.parse_input(" [TEAM] 20 [SECOND_BASE] Rob");
            let _ = parser.parse_input("inson Canó [TEAM] 14");
            let _ = parser.parse_input("7 [THIRD_BASE] DJ LeMahieu [FIRST_BA");
            let _ = parser.parse_input("SE] Anthony Rizzo [");
            let _ = parser.parse_input("GAME_START] [INNING] 1 t");
            let _ = parser.parse_input("op [PLAY] Line");
            let _ = parser.parse_input("out [BATTER] Anthony Volp");
            let _ = parser.parse_input("e [PITCHER] Trevor Bauer [FIELDERS] Aristides Aquino");
            let _ = parser.parse_input(", Kris Bry");
            let _ = parser.parse_input("ant [MOVEMENTS] Anthony Volpe home");
            let _ = parser.parse_input(" -> home");
            let _ = parser.parse_input(" [out];");

            if let Some(game_pk) = parser.game_builder.game_pk {
                assert_eq!(game_pk, 766493);
            } else {
                panic!("game_pk is None");
            }

            if let Some(date) = parser.game_builder.date {
                assert_eq!(date, "2024-03-24");
            } else {
                panic!("date is None");
            }

            if let Some(venue) = parser.game_builder.venue {
                assert_eq!(venue, "Estadio Alfredo Harp Helu");
            } else {
                panic!("venue is None");
            }

            if let Some(weather_condition) = parser.game_builder.weather_condition {
                assert_eq!(weather_condition, "Sunny");
            } else {
                panic!("weather_condition is None");
            }

            if let Some(weather_temperature) = parser.game_builder.weather_temperature {
                assert_eq!(weather_temperature, 85);
            } else {
                panic!("weather_temperature is None");
            }

            if let Some(weather_wind_speed) = parser.game_builder.weather_wind_speed {
                assert_eq!(weather_wind_speed, 91);
            } else {
                panic!("weather_wind_speed is None");
            }

            if let Some(home_team_id) = parser.game_builder.home_team_id {
                assert_eq!(home_team_id, 20);
            } else {
                panic!("home_team_id is None");
            }

            assert!(parser.game_builder.home_team_players.len() == 1);
            assert_eq!(parser.game_builder.home_team_players[0].position, Position::SecondBase);
            assert_eq!(parser.game_builder.home_team_players[0].name, "Robinson Canó");

            if let Some(away_team_id) = parser.game_builder.away_team_id {
                assert_eq!(away_team_id, 147);
            } else {
                panic!("away_team_id is None");
            }

            assert!(parser.game_builder.away_team_players.len() == 2);
            assert_eq!(parser.game_builder.away_team_players[0].position, Position::ThirdBase);
            assert_eq!(parser.game_builder.away_team_players[0].name, "DJ LeMahieu");
            assert_eq!(parser.game_builder.away_team_players[1].position, Position::FirstBase);
            assert_eq!(parser.game_builder.away_team_players[1].name, "Anthony Rizzo");

            assert!(parser.game_builder.plays.len() == 1);
            // println!("play: {:#?}", parser.game_builder.plays[0]);
            assert!(parser.game_builder.plays[0].inning == Inning { number: 1, top_bottom: TopBottom::Top });
            assert!(parser.game_builder.plays[0].play_content == PlayContent::Lineout {
                batter: "Anthony Volpe".to_string(),
                pitcher: "Trevor Bauer".to_string(),
                fielders: vec![
                    "Aristides Aquino".to_string(),
                    "Kris Bryant".to_string(),
                ],
            });
            assert!(parser.game_builder.plays[0].movements == vec![
                Movement {
                    runner: "Anthony Volpe".to_string(),
                    from: Base::Home,
                    to: Base::Home,
                    out: true,
                },
            ]);
        }

        #[test]
        fn parse_full_game() {
            pyo3::prepare_freethreaded_python();

            let mut parser = Parser::new(false);
            let input = include_str!("../test_data/748231.txt");

            let _ = parser.parse_input(&input).unwrap();

            assert!(parser.finished);

            let game = parser.complete().unwrap();
            // println!("\ngame: {:#?}\n", game);
        }

        #[test]
        fn parse_full_game_broken_up() {
            use rand::Rng;

            let mut parser = Parser::new(true);
            let mut input = include_str!("../test_data/748231.txt").to_string();

            let mut rng = rand::rng();
            let mut parts = Vec::new();
            while !input.is_empty() {
                let part_size = rng.random_range(1..=10).min(input.len());
                let part = input.chars().take(part_size).collect::<String>();
                parts.push(part);

                input = input.chars().skip(part_size).collect::<String>();
            }

            for part in parts {
                println!("part: {:?}\n", part);
                let _ = parser.parse_input(&part);
                println!("=====\n");
            }

            assert!(parser.finished);

            let game = parser.complete().unwrap();
            println!("\ngame: {:#?}\n", game);
        }

        #[test]
        fn parse_all_games_broken_up() {
            use glob::glob;
            use rand::Rng;

            pyo3::prepare_freethreaded_python();

            let paths = glob("test_data/*.txt").unwrap();

            let mut parser = Parser::new(false);
            let mut rng = rand::rng();
            for path in paths {
                println!("path: {:?}", path.as_ref().unwrap());
                let mut input = std::fs::read_to_string(path.as_ref().unwrap()).unwrap();

                let mut parts = Vec::new();
                while !input.is_empty() {
                    let part_size = rng.random_range(1..=10).min(input.len());
                    let part = input.chars().take(part_size).collect::<String>();
                    parts.push(part);

                    input = input.chars().skip(part_size).collect::<String>();
                }

                for part in parts {
                    let _ = parser.parse_input(&part).unwrap();
                }

                assert!(parser.finished);

                let game = parser.complete().unwrap();
                println!("\ngame: {:#?}\n", game);
            }
        }

        #[test]
        fn test_valid_pinch_runner() {
            let mut parser = Parser::new(false);
            let input = "[GAME] 0 [DATE] 0000-00-00 [VENUE] example [WEATHER] example 0 0\n\n[TEAM] 1\n[PITCHER] Person A\n[PINCH_RUNNER] Person B\n\n[TEAM] 2\n[PITCHER] Person C\n\n[GAME_START]\n[INNING] 1 top [PLAY] Single [BATTER] Person D [PITCHER] Person E [MOVEMENTS] Person D home -> 1;\n[INNING] 1 top [PLAY] Single [BATTER] Person Z [PITCHER] Person E [MOVEMENTS] Person Z home -> 1, Person B 1 -> 2;\n[GAME_END]";

            let result = parser.parse_input(input);

            assert!(parser.finished);
            assert!(result.is_ok());
        }

        #[test]
        fn test_invalid_pinch_runner() {
            let mut parser = Parser::new(false);
            let input = "[GAME] 0 [DATE] 0000-00-00 [VENUE] example [WEATHER] example 0 0\n\n[TEAM] 1\n[PITCHER] Person A\n\n[TEAM] 2\n[PITCHER] Person C\n\n[GAME_START]\n[INNING] 1 top [PLAY] Single [BATTER] Person D [PITCHER] Person E [MOVEMENTS] Person D home -> 1;\n[INNING] 1 top [PLAY] Single [BATTER] Person Z [PITCHER] Person E [MOVEMENTS] Person Z home -> 1, Person B 1 -> 2;\n[GAME_END]";

            println!("input: {}\n\n=====\n\n", input);
            let result = parser.parse_input(input);

            assert!(result.is_err());
        }

        #[test]
        fn simplify_movements() {
            let mut runner_positions = RunnerPositions::empty();
            runner_positions.home = Some("Garrett Hampson".to_string());
            runner_positions.first = Some("Cam Devanney".to_string());
            runner_positions.third = Some("Freddy Fermin".to_string());

            let movements = vec![
                Movement { runner: "Freddy Fermin".to_string(), from: Base::Third, to: Base::Home, out: false },
                Movement { runner: "Cam Devanney".to_string(), from: Base::First, to: Base::Second, out: false },
                Movement { runner: "Garrett Hampson".to_string(), from: Base::Home, to: Base::Home, out: true },
            ];

            let simplified_movements = runner_positions.simplify_movements(&movements);
            assert_eq!(HashSet::<_>::from_iter(simplified_movements), HashSet::from([
                Movement { runner: "Freddy Fermin".to_string(), from: Base::Third, to: Base::Home, out: false },
                Movement { runner: "Cam Devanney".to_string(), from: Base::First, to: Base::Second, out: false },
                Movement { runner: "Garrett Hampson".to_string(), from: Base::Home, to: Base::Home, out: true },
            ]));
        }
    }

    mod regex_tests {
        use super::*;

        fn test_valid_regex_for_play_type(play_type: PlayType, input: &str) {
            let parser = Parser::new(false);
            let pattern = parser.inner_pattern_from_play_type(&play_type);
            let regex = Regex::new(&pattern).unwrap();
            println!("pattern: \"{}\"\n", pattern);

            let is_match = regex.is_match(input).unwrap();
            assert!(is_match);
        }

        #[test]
        fn test_valid_regex_for_groundout() {
            test_valid_regex_for_play_type(
                PlayType::Groundout,
                "[PLAY] Groundout [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_bunt_groundout() {
            test_valid_regex_for_play_type(
                PlayType::BuntGroundout,
                "[PLAY] Bunt Groundout [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_strikeout() {
            test_valid_regex_for_play_type(
                PlayType::Strikeout,
                "[PLAY] Strikeout [BATTER] A [PITCHER] B",
            );
        }

        #[test]
        fn test_valid_regex_for_lineout() {
            test_valid_regex_for_play_type(
                PlayType::Lineout,
                "[PLAY] Lineout [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_bunt_lineout() {
            test_valid_regex_for_play_type(
                PlayType::BuntLineout,
                "[PLAY] Bunt Lineout [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_flyout() {
            test_valid_regex_for_play_type(
                PlayType::Flyout,
                "[PLAY] Flyout [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_pop_out() {
            test_valid_regex_for_play_type(
                PlayType::PopOut,
                "[PLAY] Pop Out [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_bunt_pop_out() {
            test_valid_regex_for_play_type(
                PlayType::BuntPopOut,
                "[PLAY] Bunt Pop Out [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_forceout() {
            test_valid_regex_for_play_type(
                PlayType::Forceout,
                "[PLAY] Forceout [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_fielders_choice_out() {
            test_valid_regex_for_play_type(
                PlayType::FieldersChoiceOut,
                "[PLAY] Fielders Choice Out [BATTER] A [PITCHER] B [FIELDERS] C, D [SCORING_RUNNER] E",
            );
        }

        #[test]
        fn test_valid_regex_for_double_play() {
            test_valid_regex_for_play_type(
                PlayType::DoublePlay,
                "[PLAY] Double Play [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_triple_play() {
            test_valid_regex_for_play_type(
                PlayType::TriplePlay,
                "[PLAY] Triple Play [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_runner_double_play() {
            test_valid_regex_for_play_type(
                PlayType::RunnerDoublePlay,
                "[PLAY] Runner Double Play [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_runner_triple_play() {
            test_valid_regex_for_play_type(
                PlayType::RunnerTriplePlay,
                "[PLAY] Runner Triple Play [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_grounded_into_double_play() {
            test_valid_regex_for_play_type(
                PlayType::GroundedIntoDoublePlay,
                "[PLAY] Grounded Into Double Play [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_strikeout_double_play() {
            test_valid_regex_for_play_type(
                PlayType::StrikeoutDoublePlay,
                "[PLAY] Strikeout Double Play [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_pickoff() {
            test_valid_regex_for_play_type(
                PlayType::Pickoff,
                "[PLAY] Pickoff [BASE] 1 [FIELDERS] C, D [RUNNER] E",
            );
        }

        #[test]
        fn test_valid_regex_for_pickoff_error() {
            test_valid_regex_for_play_type(
                PlayType::PickoffError,
                "[PLAY] Pickoff Error [BASE] 1 [FIELDERS] C, D [RUNNER] E",
            );
        }

        #[test]
        fn test_valid_regex_for_caught_stealing() {
            test_valid_regex_for_play_type(
                PlayType::CaughtStealing,
                "[PLAY] Caught Stealing [BASE] 1 [FIELDERS] C, D [RUNNER] E",
            );
        }

        #[test]
        fn test_valid_regex_for_pickoff_caught_stealing() {
            test_valid_regex_for_play_type(
                PlayType::PickoffCaughtStealing,
                "[PLAY] Pickoff Caught Stealing [BASE] 1 [FIELDERS] C, D [RUNNER] E",
            );
        }

        #[test]
        fn test_valid_regex_for_wild_pitch() {
            test_valid_regex_for_play_type(
                PlayType::WildPitch,
                "[PLAY] Wild Pitch [PITCHER] A [RUNNER] B",
            );
        }

        #[test]
        fn test_valid_regex_for_runner_out() {
            test_valid_regex_for_play_type(
                PlayType::RunnerOut,
                "[PLAY] Runner Out [FIELDERS] C, D [RUNNER] E",
            );
        }

        #[test]
        fn test_valid_regex_for_field_out() {
            test_valid_regex_for_play_type(
                PlayType::FieldOut,
                "[PLAY] Field Out [FIELDERS] C, D [RUNNER] E",
            );
        }

        #[test]
        fn test_valid_regex_for_batter_out() {
            test_valid_regex_for_play_type(
                PlayType::BatterOut,
                "[PLAY] Batter Out [BATTER] A [CATCHER] B",
            );
        }

        #[test]
        fn test_valid_regex_for_balk() {
            test_valid_regex_for_play_type(
                PlayType::Balk,
                "[PLAY] Balk [PITCHER] A",
            );
        }

        #[test]
        fn test_valid_regex_for_passed_ball() {
            test_valid_regex_for_play_type(
                PlayType::PassedBall,
                "[PLAY] Passed Ball [PITCHER] A [CATCHER] B",
            );
        }

        #[test]
        fn test_valid_regex_for_error() {
            test_valid_regex_for_play_type(
                PlayType::Error,
                "[PLAY] Error [PITCHER] A [CATCHER] B",
            );
        }

        #[test]
        fn test_valid_regex_for_single() {
            test_valid_regex_for_play_type(
                PlayType::Single,
                "[PLAY] Single [BATTER] A [PITCHER] B",
            );
        }

        #[test]
        fn test_valid_regex_for_double() {
            test_valid_regex_for_play_type(
                PlayType::Double,
                "[PLAY] Double [BATTER] A [PITCHER] B",
            );
        }

        #[test]
        fn test_valid_regex_for_triple() {
            test_valid_regex_for_play_type(
                PlayType::Triple,
                "[PLAY] Triple [BATTER] A [PITCHER] B",
            );
        }

        #[test]
        fn test_valid_regex_for_home_run() {
            test_valid_regex_for_play_type(
                PlayType::HomeRun,
                "[PLAY] Home Run [BATTER] A [PITCHER] B",
            );
        }

        #[test]
        fn test_valid_regex_for_walk() {
            test_valid_regex_for_play_type(
                PlayType::Walk,
                "[PLAY] Walk [BATTER] A [PITCHER] B",
            );
        }

        #[test]
        fn test_valid_regex_for_intent_walk() {
            test_valid_regex_for_play_type(
                PlayType::IntentWalk,
                "[PLAY] Intent Walk [BATTER] A [PITCHER] B",
            );
        }

        #[test]
        fn test_valid_regex_for_hit_by_pitch() {
            test_valid_regex_for_play_type(
                PlayType::HitByPitch,
                "[PLAY] Hit By Pitch [BATTER] A [PITCHER] B",
            );
        }

        #[test]
        fn test_valid_regex_for_fielders_choice() {
            test_valid_regex_for_play_type(
                PlayType::FieldersChoice,
                "[PLAY] Fielders Choice [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_catcher_interference() {
            test_valid_regex_for_play_type(
                PlayType::CatcherInterference,
                "[PLAY] Catcher Interference [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_stolen_base() {
            test_valid_regex_for_play_type(
                PlayType::StolenBase,
                "[PLAY] Stolen Base [BASE] 1 [RUNNER] A",
            );
        }

        #[test]
        fn test_valid_regex_for_sac_fly() {
            test_valid_regex_for_play_type(
                PlayType::SacFly,
                "[PLAY] Sac Fly [BATTER] A [PITCHER] B [FIELDERS] C, D [SCORING_RUNNER] E",
            );
        }

        #[test]
        fn test_valid_regex_for_sac_fly_double_play() {
            test_valid_regex_for_play_type(
                PlayType::SacFlyDoublePlay,
                "[PLAY] Sac Fly Double Play [BATTER] A [PITCHER] B [FIELDERS] C, D [SCORING_RUNNER] E",
            );
        }

        #[test]
        fn test_valid_regex_for_sac_bunt() {
            test_valid_regex_for_play_type(
                PlayType::SacBunt,
                "[PLAY] Sac Bunt [BATTER] A [PITCHER] B [FIELDERS] C, D [RUNNER] E",
            );
        }

        #[test]
        fn test_valid_regex_for_sac_bunt_double_play() {
            test_valid_regex_for_play_type(
                PlayType::SacBuntDoublePlay,
                "[PLAY] Sac Bunt Double Play [BATTER] A [PITCHER] B [FIELDERS] C, D [RUNNER] E",
            );
        }

        #[test]
        fn test_valid_regex_for_field_error() {
            test_valid_regex_for_play_type(
                PlayType::FieldError,
                "[PLAY] Field Error [BATTER] A [PITCHER] B [FIELDERS] C, D",
            );
        }

        #[test]
        fn test_valid_regex_for_game_advisory() {
            test_valid_regex_for_play_type(
                PlayType::GameAdvisory,
                "[PLAY] Game Advisory",
            );
        }

        #[test]
        fn test_valid_regex_for_movement_from_home() {
            let mut parser = Parser::new(false);
            let regex = parser.movements_regex();
            let regex = Regex::new(&regex).unwrap();

            let input = "[MOVEMENTS] A home -> 1";
            let is_match = regex.is_match(input).unwrap();
            assert!(is_match);
        }

        #[test]
        fn test_valid_regex_for_movement_from_first() {
            let mut parser = Parser::new(false);
            parser.live_game_state.runner_positions.first = Some("B".to_string());

            let regex = parser.movements_regex();
            let regex = Regex::new(&regex).unwrap();

            let input = "[MOVEMENTS] B 1 -> 2";
            let is_match = regex.is_match(input).unwrap();
            assert!(is_match);
        }

        #[test]
        fn test_valid_regex_for_movement_from_first_with_out() {
            let mut parser = Parser::new(false);
            parser.live_game_state.runner_positions.first = Some("B".to_string());

            let regex = parser.movements_regex();
            let regex = Regex::new(&regex).unwrap();

            let input = "[MOVEMENTS] B 1 -> 2 [out]";
            let is_match = regex.is_match(input).unwrap();
            assert!(is_match);
        }

        #[test]
        fn test_valid_regex_for_multiple_movements() {
            let mut parser = Parser::new(false);
            parser.live_game_state.runner_positions.first = Some("B".to_string());

            let regex = parser.movements_regex();
            let regex = Regex::new(&regex).unwrap();

            let input = "[MOVEMENTS] A home -> 1, B 1 -> 2 [out]";
            let is_match = regex.is_match(input).unwrap();
            assert!(is_match);
        }
    }
}
