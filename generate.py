from mlb_parser import Parser, get_next_valid_chars

import random


parser = Parser(False)

game = ""
while not parser.finished:
    full_pattern = parser.valid_regex()
    print(repr(full_pattern))
    valid_chars = get_next_valid_chars(game, full_pattern)
    print(f"\n\nValid chars: {valid_chars}")
    char = random.choice(valid_chars)
    game += char
    print(f"\n\nGenerated: {repr(char)}\t{repr(game)}")

    parser.parse_input(char)
    # input()
