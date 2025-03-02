# game data format

preprocessed play-by-play data is stored in a custom format for this project.

## overview

at a high level, each game file contains the following sections:

1. **metadata**: game pk, date, venue, and weather.
2. **home team data**: team id and players.
3. **away team data**: team id and players.
4. **plays**: a list of plays.

a game ends with the `[GAME_END]` tag.

## metadata

a game pk is `[GAME]` followed by an integer. a date is `[DATE]` followed by a string in the format `YYYY-MM-DD`. a venue is `[VENUE]` followed bya string of one or more words. weather is `[WEATHER]` followed by a string of one or more words, representing the weather condition, followed by an integer representing the temperature in fahrenheit, followed by an integer representing the wind speed in mph.

## team data

a team id is `[TEAM]` followed by an integer. a player is a player type tag followed by a player's name. valid player type tags are `[PITCHER]`, `[CATCHER]`, `[FIRST_BASE]`, `[SECOND_BASE]`, `[THIRD_BASE]`, `[SHORTSTOP]`, `[LEFT_FIELD]`, `[CENTER_FIELD]`, `[RIGHT_FIELD]`, `[DESIGNATED_HITTER]`, `[PINCH_HITTER]`, `[PINCH_RUNNER]`, `[TWO_WAY_PLAYER]`, `[OUTFIELD]`, `[INFIELD]`, `[UTILITY]`, `[RELIEF_PITCHER]`, and `[STARTING_PITCHER]`.

## plays

a play contains the following data:

1. **inning**: `[INNING]` followed by an integer representing the inning and either `top` or `bottom`.
2. **play type**: `[PLAY]` followed by a string representing the play type.
3. **base (optional)**: `[BASE]` followed by an integer representing the base the play occurred at.
4. **players**: a list of players involved in the play. a player is a player type tag followed by a player's name.
5. **movements**: `[MOVEMENTS]` followed by a comma-separated list of movements made during the play.

valid play types are `Groundout`, `Bunt Groundout`, `Strikeout`, `Lineout`, `Bunt Lineout`, `Flyout`, `Pop Out`, `Bunt Pop Out`, `Forceout`, `Fielders Choice Out`, `Double Play`, `Triple Play`, `Runner Double Play`, `Runner Triple Play`, `Grounded Into Double Play`, `Strikeout Double Play`, `Pickoff`, `Pickoff Error`, `Caught Stealing`, `Pickoff Caught Stealing`, `Wild Pitch`, `Runner Out`, `Field Out`, `Batter Out`, `Balk`, `Passed Ball`, `Error`, `Single`, `Double`, `Triple`, `Home Run`, `Walk`, `Intent Walk`, `Hit By Pitch`, `Fielders Choice`, `Catcher Interference`, `Stolen Base`, `Sac Fly`, `Sac Fly Double Play`, `Sac Bunt`, `Sac Bunt Double Play`, `Field Error`, and `Game Advisory`.

possible player types in a play are `[BATTER]`, `[PITCHER]`, `[CATCHER]`, `[FIELDERS]`, `[RUNNER]`, and `[SCORING_RUNNER]`.

a movement is a player's name followed by their starting base, `->`, and their ending base. if a player is out, the movement is followed by `[out]`.

below is a table of what information is required for each play type:

| Play Type | Base | Batter | Pitcher | Catcher | Fielders | Runner | Scoring Runner |
|-|-|-|-|-|-|-|-|
| Groundout | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Bunt Groundout | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Strikeout | ✗ | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ |
| Lineout | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Bunt Lineout | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Flyout | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Pop Out | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Bunt Pop Out | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Forceout | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Fielders Choice Out | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✓ |
| Double Play | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Triple Play | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Runner Double Play | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Runner Triple Play | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Grounded Into Double Play | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Strikeout Double Play | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Pickoff | ✓ | ✗ | ✗ | ✗ | ✓ | ✓ | ✗ |
| Pickoff Error | ✓ | ✗ | ✗ | ✗ | ✓ | ✓ | ✗ |
| Caught Stealing | ✓ | ✗ | ✗ | ✗ | ✓ | ✓ | ✗ |
| Pickoff Caught Stealing | ✓ | ✗ | ✗ | ✗ | ✓ | ✓ | ✗ |
| Wild Pitch | ✗ | ✗ | ✓ | ✗ | ✗ | ✓ | ✗ |
| Runner Out | ✗ | ✗ | ✗ | ✗ | ✓ | ✓ | ✗ |
| Field Out | ✗ | ✗ | ✗ | ✗ | ✓ | ✓ | ✗ |
| Batter Out | ✗ | ✓ | ✗ | ✓ | ✗ | ✗ | ✗ |
| Balk | ✗ | ✗ | ✓ | ✗ | ✗ | ✗ | ✗ |
| Passed Ball | ✗ | ✗ | ✓ | ✓ | ✗ | ✗ | ✗ |
| Error | ✗ | ✗ | ✓ | ✓ | ✗ | ✗ | ✗ |
| Single | ✗ | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ |
| Double | ✗ | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ |
| Triple | ✗ | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ |
| Home Run | ✗ | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ |
| Walk | ✗ | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ |
| Intent Walk | ✗ | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ |
| Hit By Pitch | ✗ | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ |
| Fielders Choice | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Catcher Interference | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Stolen Base | ✓ | ✗ | ✗ | ✗ | ✗ | ✓ | ✗ |
| Sac Fly | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✓ |
| Sac Fly Double Play | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✓ |
| Sac Bunt | ✗ | ✓ | ✓ | ✗ | ✓ | ✓ | ✗ |
| Sac Bunt Double Play | ✗ | ✓ | ✓ | ✗ | ✓ | ✓ | ✗ |
| Field Error | ✗ | ✓ | ✓ | ✗ | ✓ | ✗ | ✗ |
| Game Advisory | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ |
