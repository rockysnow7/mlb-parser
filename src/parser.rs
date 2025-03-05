mod game;

use game::{Base, Game, GameBuilder, Inning, PlayType, Player, Position, TopBottom};
use once_cell::sync::Lazy;
use pyo3::prelude::{PyResult, pyclass, pymethods};
use fancy_regex::Regex;
use strum::IntoEnumIterator;
use std::collections::HashSet;

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

const BASE_NAME: &str = "1|2|3|4|home";
static BASE_NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(format!(
    r"^({})",
    BASE_NAME,
).as_str()).unwrap());
const COMMA_SPACE: &str = ", ";
const PLAYER_NAME: &str = "[a-zA-ZÀ-ÖØ-öø-ÿ.' ]+";
static PLAYER_NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(PLAYER_NAME).unwrap());

static CONTEXT_SECTION_GAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[GAME\] (?P<game_pk>\d+)").unwrap());
static CONTEXT_SECTION_DATE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[DATE\] (?P<date>\d{4}-\d{2}-\d{2})").unwrap());
static CONTEXT_SECTION_VENUE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[VENUE\] (?P<venue>[a-zA-ZÀ-ÖØ-öø-ÿ ]+)").unwrap());
static CONTEXT_SECTION_WEATHER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[WEATHER\] (?P<weather>[a-zA-ZÀ-ÖØ-öø-ÿ ]+) (?P<temperature>\d+) (?P<wind_speed>\d+)").unwrap());

static TEAM_SECTION_TEAM_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[TEAM\] (?P<team_id>\d+)").unwrap());
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

static PLAY_SECTION_INNING_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[INNING\] (?P<number>\d+) (?P<top_bottom>top|bottom)").unwrap());
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

#[pyclass]
pub struct Parser {
    input_buffer: String,
    possible_sections: Vec<GameSection>,
    game_builder: GameBuilder,
    finished: bool,
    print_debug: bool,
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
        }
    }

    fn consume_input(&mut self, index: usize) {
        self.input_buffer = self.input_buffer
            .split_off(index)
            .trim_start()
            .to_string();
    }

    fn parse_context_section(&mut self, context_section: ContextSection) -> PyResult<(bool, HashSet<char>)> {
        match context_section {
            ContextSection::Game => {
                if self.print_debug {
                    println!("Parsing ContextSection::Game");
                }

                let captures = CONTEXT_SECTION_GAME_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let game_pk_match = captures.name("game_pk").unwrap();
                    let game_pk = game_pk_match.as_str().parse::<u64>().unwrap();
                    self.game_builder.set_game_pk(game_pk);

                    self.consume_input(game_pk_match.end());
                    self.possible_sections = vec![GameSection::Context(ContextSection::Date)];

                    return Ok((true, HashSet::new()));
                }
            },
            ContextSection::Date => {
                if self.print_debug {
                    println!("Parsing ContextSection::Date");
                }

                let captures = CONTEXT_SECTION_DATE_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let date_match = captures.name("date").unwrap();
                    let date = date_match.as_str().to_string();
                    self.game_builder.set_date(date);

                    self.consume_input(date_match.end());
                    self.possible_sections = vec![GameSection::Context(ContextSection::Venue)];

                    return Ok((true, HashSet::new()));
                }
            },
            ContextSection::Venue => {
                if self.print_debug {
                    println!("Parsing ContextSection::Venue");
                }

                let captures = CONTEXT_SECTION_VENUE_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let venue_match = captures.name("venue").unwrap();
                    let venue = venue_match.as_str().trim().to_string();
                    self.game_builder.set_venue(venue);

                    self.consume_input(venue_match.end());
                    self.possible_sections = vec![GameSection::Context(ContextSection::Weather)];

                    return Ok((true, HashSet::new()));
                }
            },
            ContextSection::Weather => {
                if self.print_debug {
                    println!("Parsing ContextSection::Weather");
                }

                let captures = CONTEXT_SECTION_WEATHER_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let weather_match = captures.name("weather").unwrap();
                    let weather = weather_match.as_str().to_string();

                    let temperature_match = captures.name("temperature").unwrap();
                    let temperature = temperature_match.as_str().parse::<u64>().unwrap();

                    let wind_speed_match = captures.name("wind_speed").unwrap();
                    let wind_speed = wind_speed_match.as_str().parse::<u64>().unwrap();

                    self.game_builder.set_weather(weather, temperature, wind_speed);

                    self.consume_input(wind_speed_match.end());
                    self.possible_sections = vec![GameSection::HomeTeam(TeamSection::Team)];

                    return Ok((true, HashSet::new()));
                }
            },
        }

        Ok((false, HashSet::new()))
    }

    fn parse_team_section(&mut self, team_section: TeamSection, home_team: bool) -> PyResult<(bool, HashSet<char>)> {
        match team_section {
            TeamSection::Team => {
                if self.print_debug {
                    println!("Parsing TeamSection::Team");
                }

                let captures = TEAM_SECTION_TEAM_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let team_id_match = captures.name("team_id").unwrap();
                    let team_id = team_id_match.as_str().parse::<u64>().unwrap();

                    if home_team {
                        self.game_builder.set_home_team_id(team_id);
                    } else {
                        self.game_builder.set_away_team_id(team_id);
                    }

                    self.consume_input(team_id_match.end());

                    if home_team {
                        self.possible_sections = vec![GameSection::HomeTeam(TeamSection::Player)];
                    } else {
                        self.possible_sections = vec![GameSection::AwayTeam(TeamSection::Player)];
                    }

                    return Ok((true, HashSet::new()));
                }
            },
            TeamSection::Player => {
                if self.print_debug {
                    println!("Parsing TeamSection::Player");
                }

                let captures = TEAM_SECTION_PLAYER_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let position_match = captures.name("position").unwrap();
                    let position = position_match.as_str().parse::<Position>().unwrap();

                    let player_name_match = captures.name("player_name").unwrap();
                    let player_name = player_name_match.as_str().trim().to_string();

                    let player = Player {
                        position,
                        name: player_name,
                    };

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

                    return Ok((true, HashSet::new()));
                }
            },
        }

        Ok((false, HashSet::new()))
    }

    fn parse_play_section(&mut self, play_section: PlaySection) -> PyResult<(bool, HashSet<char>)> {
        match play_section {
            PlaySection::GameStart() => {
                if self.print_debug {
                    println!("Parsing PlaySection::GameStart");
                }

                if self.input_buffer.starts_with("[GAME_START]") {
                    self.consume_input("[GAME_START]".len());
                    self.possible_sections = vec![GameSection::Plays(PlaySection::Inning())];

                    return Ok((true, HashSet::new()));
                }
            },
            PlaySection::Inning() => {
                if self.print_debug {
                    println!("Parsing PlaySection::Inning");
                }

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

                    self.consume_input(top_bottom_match.end());
                    self.possible_sections = vec![GameSection::Plays(PlaySection::Play())];

                    return Ok((true, HashSet::new()));
                }
            },
            PlaySection::Play() => {
                if self.print_debug {
                    println!("Parsing PlaySection::Play");
                }


                let captures = PLAY_SECTION_PLAY_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let play_type_match = captures.name("play_type").unwrap();
                    let play_type = play_type_match.as_str().parse::<PlayType>().unwrap();

                    self.game_builder.play_builder.set_play_type(play_type);

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

                    return Ok((true, HashSet::new()));
                }
            },
            PlaySection::Base() => {
                if self.print_debug {
                    println!("Parsing PlaySection::Base");
                }

                let captures = PLAY_SECTION_BASE_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let base_match = captures.name("base").unwrap();
                    let base = base_match.as_str().parse::<Base>().unwrap();

                    self.game_builder.play_builder.set_base(base);

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

                    return Ok((true, HashSet::new()));
                }
            },
            PlaySection::Batter() => {
                if self.print_debug {
                    println!("Parsing PlaySection::Batter");
                }

                let captures = PLAY_SECTION_BATTER_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let batter_match = captures.name("batter").unwrap();
                    let batter = batter_match.as_str().trim().to_string();

                    self.game_builder.play_builder.set_batter(batter);

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

                    return Ok((true, HashSet::new()));
                }
            },
            PlaySection::Pitcher() => {
                if self.print_debug {
                    println!("Parsing PlaySection::Pitcher");
                }

                let captures = PLAY_SECTION_PITCHER_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let pitcher_match = captures.name("pitcher").unwrap();
                    let pitcher = pitcher_match.as_str().trim().to_string();

                    self.game_builder.play_builder.set_pitcher(pitcher);

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

                    return Ok((true, HashSet::new()));
                }
            },
            PlaySection::Catcher() => {
                if self.print_debug {
                    println!("Parsing PlaySection::Catcher");
                }

                let captures = PLAY_SECTION_CATCHER_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let catcher_match = captures.name("catcher").unwrap();
                    let catcher = catcher_match.as_str().trim().to_string();

                    self.game_builder.play_builder.set_catcher(catcher);

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

                    return Ok((true, HashSet::new()));
                }
            },
            PlaySection::Fielders(fielders_section) => {
                if self.print_debug {
                    println!("Parsing PlaySection::Fielders");
                }

                match fielders_section {
                    FieldersSection::Tag => {
                        if self.print_debug {
                            println!("\t(=> FieldersSection::Tag)");
                        }

                        if self.input_buffer.starts_with(PLAY_SECTION_FIELDERS_TAG) {
                            self.consume_input(PLAY_SECTION_FIELDERS_TAG.len());
                            self.possible_sections = vec![GameSection::Plays(PlaySection::Fielders(FieldersSection::Name))];

                            return Ok((true, HashSet::new()));
                        }
                    },
                    FieldersSection::Name => {
                        if self.print_debug {
                            println!("\t(=> FieldersSection::Name)");
                        }

                        let mut matches = PLAYER_NAME_REGEX.find_iter(&self.input_buffer);
                        let player_name_match = matches.next();
                        if let Some(Ok(player_name_match)) = player_name_match {
                            let player_name = player_name_match.as_str().trim().to_string();

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

                            return Ok((true, HashSet::new()));
                        }
                    },
                    FieldersSection::CommaSpace => {
                        if self.print_debug {
                            println!("\t(=> FieldersSection::CommaSpace)");
                        }

                        if self.input_buffer.starts_with(COMMA_SPACE) {
                            self.consume_input(COMMA_SPACE.len());
                            self.possible_sections = vec![GameSection::Plays(PlaySection::Fielders(FieldersSection::Name))];

                            return Ok((true, HashSet::new()));
                        }
                    },
                }
            },
            PlaySection::Runner() => {
                if self.print_debug {
                    println!("Parsing PlaySection::Runner");
                }

                let captures = PLAY_SECTION_RUNNER_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let runner_match = captures.name("runner").unwrap();
                    let runner = runner_match.as_str().trim().to_string();

                    self.game_builder.play_builder.set_runner(runner);

                    self.consume_input(runner_match.end());

                    let play_type = self.game_builder.play_builder.play_type.unwrap();
                    if play_type.requires_scoring_runner() {
                        self.possible_sections = vec![GameSection::Plays(PlaySection::ScoringRunner())];
                    } else if play_type.requires_fielders() {
                        self.possible_sections = vec![GameSection::Plays(PlaySection::Fielders(FieldersSection::Tag))];
                    } else {
                        self.possible_sections = vec![GameSection::Plays(PlaySection::Movements(MovementsSection::Tag))];
                    }

                    return Ok((true, HashSet::new()));
                }
            },
            PlaySection::ScoringRunner() => {
                if self.print_debug {
                    println!("Parsing PlaySection::ScoringRunner");
                }

                let captures = PLAY_SECTION_SCORING_RUNNER_REGEX.captures(&self.input_buffer);
                if let Ok(Some(captures)) = captures {
                    let scoring_runner_match = captures.name("scoring_runner").unwrap();
                    let scoring_runner = scoring_runner_match.as_str().trim().to_string();

                    self.game_builder.play_builder.set_scoring_runner(scoring_runner);

                    self.consume_input(scoring_runner_match.end());
                    self.possible_sections = vec![GameSection::Plays(PlaySection::Movements(MovementsSection::Tag))];

                    return Ok((true, HashSet::new()));
                }
            },
            PlaySection::Movements(movements_section) => {
                if self.print_debug {
                    println!("Parsing PlaySection::Movements");
                }

                match movements_section {
                    MovementsSection::Tag => {
                        if self.print_debug {
                            println!("\t(=> MovementsSection::Tag)");
                        }

                        if self.input_buffer.starts_with(PLAY_SECTION_MOVEMENTS_TAG) {
                            self.consume_input(PLAY_SECTION_MOVEMENTS_TAG.len());
                            self.possible_sections = vec![GameSection::Plays(PlaySection::Movements(MovementsSection::Name))];

                            return Ok((true, HashSet::new()));
                        }
                    },
                    MovementsSection::Name => {
                        if self.print_debug {
                            println!("\t(=> MovementsSection::Name)");
                        }

                        let mut matches = PLAYER_NAME_REGEX.find_iter(&self.input_buffer);
                        let player_name_match = matches.next();
                        if let Some(Ok(player_name_match)) = player_name_match {
                            let mut player_name = player_name_match.as_str().trim().to_string();
                            let mut trimmed_length = 0;

                            for base in ["1", "2", "3", "4", "home"] {
                                if player_name.ends_with(base) {
                                    trimmed_length = base.len();
                                    player_name.truncate(player_name.len() - trimmed_length);
                                    break;
                                }
                            }

                            player_name = player_name.trim().to_string();
                            self.game_builder.play_builder.movement_builder.set_runner(player_name);

                            self.consume_input(player_name_match.end() - trimmed_length - 1);
                            self.possible_sections = vec![GameSection::Plays(PlaySection::Movements(MovementsSection::StartBase))];

                            return Ok((true, HashSet::new()));
                        }
                    },
                    MovementsSection::StartBase => {
                        if self.print_debug {
                            println!("\t(=> MovementsSection::StartBase)");
                        }

                        let mut matches = BASE_NAME_REGEX.find_iter(&self.input_buffer);
                        let base_match = matches.next();
                        if let Some(Ok(base_match)) = base_match {
                            let base = base_match.as_str().trim().parse::<Base>().unwrap();

                            self.game_builder.play_builder.movement_builder.set_from(base);

                            self.consume_input(base_match.end());
                            self.possible_sections = vec![GameSection::Plays(PlaySection::Movements(MovementsSection::Arrow))];

                            return Ok((true, HashSet::new()));
                        }
                    },
                    MovementsSection::Arrow => {
                        if self.print_debug {
                            println!("\t(=> MovementsSection::Arrow)");
                        }

                        if self.input_buffer.starts_with(PLAY_SECTION_ARROW) {
                            self.consume_input(PLAY_SECTION_ARROW.len());
                            self.possible_sections = vec![GameSection::Plays(PlaySection::Movements(MovementsSection::EndBase))];

                            return Ok((true, HashSet::new()));
                        }
                    },
                    MovementsSection::EndBase => {
                        if self.print_debug {
                            println!("\t(=> MovementsSection::EndBase)");
                        }

                        let mut matches = BASE_NAME_REGEX.find_iter(&self.input_buffer);
                        let base_match = matches.next();
                        if let Some(Ok(base_match)) = base_match {
                            let base = base_match.as_str().trim().parse::<Base>().unwrap();

                            self.game_builder.play_builder.movement_builder.set_to(base);

                            self.consume_input(base_match.end());
                            self.possible_sections = vec![
                                GameSection::Plays(PlaySection::Movements(MovementsSection::Out)),
                                GameSection::Plays(PlaySection::Movements(MovementsSection::MovementEnd)),
                            ];

                            return Ok((true, HashSet::new()));
                        }
                    },
                    MovementsSection::Out => {
                        if self.print_debug {
                            println!("\t(=> MovementsSection::Out)");
                        }

                        if self.input_buffer.starts_with(PLAY_SECTION_OUT) {
                            self.consume_input(PLAY_SECTION_OUT.len());
                            self.game_builder.play_builder.movement_builder.set_out(true);

                            self.possible_sections = vec![
                                GameSection::Plays(PlaySection::Movements(MovementsSection::MovementEnd)),
                            ];

                            return Ok((true, HashSet::new()));
                        }
                    },
                    MovementsSection::CommaSpace => {
                        if self.print_debug {
                            println!("\t(=> MovementsSection::CommaSpace)");
                        }

                        if self.input_buffer.starts_with(COMMA_SPACE) {
                            self.consume_input(COMMA_SPACE.len());
                            self.possible_sections = vec![GameSection::Plays(PlaySection::Movements(MovementsSection::Name))];

                            return Ok((true, HashSet::new()));
                        }
                    },
                    MovementsSection::MovementEnd => {
                        if self.print_debug {
                            println!("\t(=> MovementsSection::MovementEnd)");
                        }

                        let _ = self.game_builder.play_builder.build_movement();

                        self.possible_sections = vec![
                            GameSection::Plays(PlaySection::Movements(MovementsSection::CommaSpace)),
                            GameSection::Plays(PlaySection::PlayEnd()),
                        ];

                        return Ok((true, HashSet::new()));
                    },
                }
            },
            PlaySection::PlayEnd() => {
                if self.print_debug {
                    println!("Parsing PlaySection::PlayEnd");
                }

                self.game_builder.build_play();

                self.possible_sections = vec![
                    GameSection::Plays(PlaySection::Inning()),
                    GameSection::Plays(PlaySection::GameEnd()),
                ];

                return Ok((true, HashSet::new()));
            },
            PlaySection::GameEnd() => {
                if self.print_debug {
                    println!("Parsing PlaySection::GameEnd");
                }

                if self.input_buffer.starts_with("[GAME_END]") {
                    self.consume_input("[GAME_END]".len());
                    self.finished = true;

                    return Ok((true, HashSet::new()));
                }

                return Ok((false, HashSet::new()));
            },
        }

        Ok((false, HashSet::new()))
    }

    fn parse_input_buffer(&mut self) -> PyResult<(bool, HashSet<char>)> {
        for section in self.possible_sections.clone() {
            let (success, valid_next_chars) = match section {
                GameSection::Context(context_section) => self.parse_context_section(context_section),
                GameSection::HomeTeam(team_section) => self.parse_team_section(team_section, true),
                GameSection::AwayTeam(team_section) => self.parse_team_section(team_section, false),
                GameSection::Plays(play_section) => self.parse_play_section(play_section),
            }?;

            if success {
                return Ok((success, valid_next_chars));
            }
        }

        Ok((false, HashSet::new()))
    }

    /// Stream-parse a game and return the set of valid next characters.
    pub fn parse_input(&mut self, input: &str) -> PyResult<HashSet<char>> {
        self.input_buffer.push_str(input);

        loop {
            if self.finished {
                return Ok(HashSet::new());
            }

            let (success, valid_next_chars) = self.parse_input_buffer()?;

            if !success {
                return Ok(valid_next_chars);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game_pk() {
        let mut parser = Parser::new(true);
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
        let mut parser = Parser::new(true);
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
        let mut parser = Parser::new(true);
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
        let mut parser = Parser::new(true);
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
        let mut parser = Parser::new(true);
        let input = "[GAME] 0 [DATE] 0000-00-00 [VENUE] venue [WEATHER] weather 0 0\n\n[TEAM] 20\n[SECOND_BASE] Robinson Canó\n[PITCHER] Arturo Lopez";

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
        let mut parser = Parser::new(true);
        let input = "[GAME] 0 [DATE] 0000-00-00 [VENUE] venue [WEATHER] weather 0 0\n\n[TEAM] 20\n[SECOND_BASE] Robinson Canó\n[PITCHER] Arturo Lopez [TEAM] 147 [THIRD_BASE] DJ LeMahieu [FIRST_BASE] Anthony Rizzo";

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

        let mut parser = Parser::new(true);
        let input = "[GAME] 766493 [DATE] 2024-03-24 [VENUE] Estadio Alfredo Harp Helu [WEATHER] Sunny 85 9 [TEAM] 20 [SECOND_BASE] Robinson Canó [TEAM] 147 [THIRD_BASE] DJ LeMahieu [GAME_START] [INNING] 1 top [PLAY] Lineout [BATTER] Anthony Volpe [PITCHER] Trevor Bauer [FIELDERS] Aristides Aquino [MOVEMENTS] Anthony Volpe home -> home [out]";

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
        let mut parser = Parser::new(true);
        let input = "[GAME] 766493 [DATE] 2024-03-24 [VENUE] Estadio Alfredo Harp Helu [WEATHER] Sunny 85 9 [TEAM] 20 [SECOND_BASE] Robinson Canó [TEAM] 147 [THIRD_BASE] DJ LeMahieu [GAME_START] [INNING] 3 bottom [PLAY] Groundout [BATTER] Juan Carlos Gamboa [PITCHER] Tanner Tully [FIELDERS] Tanner Tully, Trevor Bauer [MOVEMENTS] Juan Carlos Gamboa home -> home [out], Xavier Fernández 1 -> 2";

        let _ = parser.parse_input(input);

        if let Some(play) = parser.game_builder.plays.iter().next() {
            assert!(play.inning == Inning { number: 3, top_bottom: TopBottom::Bottom });
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
                    from: Base::First,
                    to: Base::Second,
                    out: false,
                },
            ]);
        } else {
            panic!("play is None");
        }
    }

    #[test]
    fn parse_full_game() {
        let re = Regex::new(r"\s+").unwrap();

        let mut parser = Parser::new(true);
        let input = include_str!("../test_data/766493.txt");
        let input = re.replace_all(input, " ");

        println!("\ninput: {}\n", input);

        let _ = parser.parse_input(&input);

        assert!(parser.finished);

        let game = parser.complete().unwrap();
        println!("\ngame: {:#?}\n", game);
    }
}
