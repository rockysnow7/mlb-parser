mod game;

use game::{Game, GameBuilder, Inning, PlayContent, PlayType, Player, Position, TopBottom};
use once_cell::sync::Lazy;
use pyo3::prelude::{PyResult, pyclass, pymethods};
use regex::Regex;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
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
enum PlaySection {
    GameStart,
    Inning,
    Play,
    Base,
    Batter,
    Pitcher,
    Catcher,
    Fielders,
    Runner,
    ScoringRunner,
    Movements,
    GameEnd,
}

#[derive(Debug, Clone, Copy, Hash)]
struct Play {
    play_type: PlayType,
    play_section: PlaySection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameSection {
    Context(ContextSection),
    HomeTeam(TeamSection),
    AwayTeam(TeamSection),
    Plays(PlaySection),
}

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
    r"^\[(?P<position>{})\] (?P<player_name>[a-zA-ZÀ-ÖØ-öø-ÿ.' ]+)",
    ALL_POSITIONS.as_str(),
).as_str()).unwrap());

static PLAY_SECTION_INNING_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\[INNING\] (?P<number>\d+) (?P<top_bottom>top|bottom)").unwrap());
static ALL_PLAY_TYPES: Lazy<String> = Lazy::new(|| {
    let mut play_types = Vec::new();
    for play_type in PlayType::iter() {
        play_types.push(play_type.to_string());
    }
    play_types.join("|")
});
static PLAY_SECTION_PLAY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(format!(
    r"^\[PLAY\] (?P<play_type>{})",
    ALL_PLAY_TYPES.as_str(),
).as_str()).unwrap());

#[pyclass]
pub struct Parser {
    input_buffer: String,
    possible_sections: HashSet<GameSection>,
    game_builder: GameBuilder,
}

#[pymethods]
impl Parser {
    #[staticmethod]
    fn new() -> Self {
        Self {
            input_buffer: String::new(),
            possible_sections: HashSet::from([GameSection::Context(ContextSection::Game)]),
            game_builder: GameBuilder::new(),
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
                println!("Parsing game section...");

                let captures = CONTEXT_SECTION_GAME_REGEX.captures(&self.input_buffer);
                if let Some(captures) = captures {
                    let game_pk_match = captures.name("game_pk").unwrap();
                    let game_pk = game_pk_match.as_str().parse::<u64>().unwrap();
                    self.game_builder.set_game_pk(game_pk);

                    self.consume_input(game_pk_match.end());
                    self.possible_sections = HashSet::from([GameSection::Context(ContextSection::Date)]);

                    return Ok((true, HashSet::new()));
                }
            },
            ContextSection::Date => {
                println!("Parsing date section...");

                let captures = CONTEXT_SECTION_DATE_REGEX.captures(&self.input_buffer);
                if let Some(captures) = captures {
                    let date_match = captures.name("date").unwrap();
                    let date = date_match.as_str().to_string();
                    self.game_builder.set_date(date);

                    self.consume_input(date_match.end());
                    self.possible_sections = HashSet::from([GameSection::Context(ContextSection::Venue)]);

                    return Ok((true, HashSet::new()));
                }
            },
            ContextSection::Venue => {
                println!("Parsing venue section...");

                let captures = CONTEXT_SECTION_VENUE_REGEX.captures(&self.input_buffer);
                if let Some(captures) = captures {
                    let venue_match = captures.name("venue").unwrap();
                    let venue = venue_match.as_str().trim().to_string();
                    self.game_builder.set_venue(venue);

                    self.consume_input(venue_match.end());
                    self.possible_sections = HashSet::from([GameSection::Context(ContextSection::Weather)]);

                    return Ok((true, HashSet::new()));
                }
            },
            ContextSection::Weather => {
                println!("Parsing weather section...");

                let captures = CONTEXT_SECTION_WEATHER_REGEX.captures(&self.input_buffer);
                if let Some(captures) = captures {
                    let weather_match = captures.name("weather").unwrap();
                    let weather = weather_match.as_str().to_string();

                    let temperature_match = captures.name("temperature").unwrap();
                    let temperature = temperature_match.as_str().parse::<u64>().unwrap();

                    let wind_speed_match = captures.name("wind_speed").unwrap();
                    let wind_speed = wind_speed_match.as_str().parse::<u64>().unwrap();

                    self.game_builder.set_weather(weather, temperature, wind_speed);

                    self.consume_input(wind_speed_match.end());
                    self.possible_sections = HashSet::from([GameSection::HomeTeam(TeamSection::Team)]);

                    return Ok((true, HashSet::new()));
                }
            },
        }

        Ok((false, HashSet::new()))
    }

    fn parse_team_section(&mut self, team_section: TeamSection, home_team: bool) -> PyResult<(bool, HashSet<char>)> {
        match team_section {
            TeamSection::Team => {
                println!("Parsing team section...");

                let captures = TEAM_SECTION_TEAM_REGEX.captures(&self.input_buffer);
                if let Some(captures) = captures {
                    let team_id_match = captures.name("team_id").unwrap();
                    let team_id = team_id_match.as_str().parse::<u64>().unwrap();

                    if home_team {
                        self.game_builder.set_home_team_id(team_id);
                    } else {
                        self.game_builder.set_away_team_id(team_id);
                    }

                    self.consume_input(team_id_match.end());

                    if home_team {
                        self.possible_sections = HashSet::from([GameSection::HomeTeam(TeamSection::Player)]);
                    } else {
                        self.possible_sections = HashSet::from([GameSection::AwayTeam(TeamSection::Player)]);
                    }

                    return Ok((true, HashSet::new()));
                }
            },
            TeamSection::Player => {
                println!("Parsing player section...");

                let captures = TEAM_SECTION_PLAYER_REGEX.captures(&self.input_buffer);
                if let Some(captures) = captures {
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
                        self.possible_sections = HashSet::from([
                            GameSection::HomeTeam(TeamSection::Player),
                            GameSection::AwayTeam(TeamSection::Team),
                        ]);
                    } else {
                        self.game_builder.add_away_team_player(player);
                        self.possible_sections = HashSet::from([
                            GameSection::AwayTeam(TeamSection::Player),
                            GameSection::Plays(PlaySection::GameStart),
                        ]);
                    }

                    return Ok((true, HashSet::new()));
                }
            },
        }

        Ok((false, HashSet::new()))
    }

    fn parse_play_section(&mut self, play_section: PlaySection) -> PyResult<(bool, HashSet<char>)> {
        match play_section {
            PlaySection::GameStart => {
                println!("Parsing game start section...");

                if self.input_buffer.starts_with("[GAME_START]") {
                    self.consume_input("[GAME_START]".len());
                    self.possible_sections = HashSet::from([GameSection::Plays(PlaySection::Inning)]);

                    return Ok((true, HashSet::new()));
                }
            },
            PlaySection::Inning => {
                println!("Parsing inning section...");

                let captures = PLAY_SECTION_INNING_REGEX.captures(&self.input_buffer);
                if let Some(captures) = captures {
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
                    self.possible_sections = HashSet::from([GameSection::Plays(PlaySection::Play)]);

                    return Ok((true, HashSet::new()));
                }
            },
            PlaySection::Play => {
                println!("Parsing play section...");

                let captures = PLAY_SECTION_PLAY_REGEX.captures(&self.input_buffer);
                if let Some(captures) = captures {
                    let play_type_match = captures.name("play_type").unwrap();
                    let play_type = play_type_match.as_str().parse::<PlayType>().unwrap();

                    self.game_builder.play_builder.set_play_type(play_type);

                    self.consume_input(play_type_match.end());

                    if play_type == PlayType::GameAdvisory {
                        self.game_builder.build_play();
                        self.possible_sections = HashSet::from([
                            GameSection::Plays(PlaySection::Inning),
                            GameSection::Plays(PlaySection::GameEnd),
                        ]);
                    } else {
                        todo!()
                    }

                    return Ok((true, HashSet::new()));
                }
            },
            _ => todo!(),
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
            let (success, valid_next_chars) = self.parse_input_buffer()?;

            if !success {
                return Ok(valid_next_chars);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game_pk() {
        let mut parser = Parser::new();
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
        let mut parser = Parser::new();
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
        let mut parser = Parser::new();
        let input = "[GAM";
        let result = parser.parse_input(input);

        assert!(result.is_ok());
        assert_eq!(parser.possible_sections, HashSet::from([GameSection::Context(ContextSection::Game)]));

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
        let mut parser = Parser::new();
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
        let mut parser = Parser::new();
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
        let mut parser = Parser::new();
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

    // #[test]
    // fn parse_one_play() {
    //     let mut parser = Parser::new();
    //     let input = "[GAME] 766493 [DATE] 2024-03-24 [VENUE] Estadio Alfredo Harp Helu [WEATHER] Sunny 85 9 [TEAM] 20 [SECOND_BASE] Robinson Canó [TEAM] 147 [THIRD_BASE] DJ LeMahieu [GAME_START] [INNING] 1 top [PLAY] Lineout [BATTER] Anthony Volpe [PITCHER] Trevor Bauer [FIELDERS] Aristides Aquino [MOVEMENTS] Anthony Volpe home -> home [out]";
    // }
}
