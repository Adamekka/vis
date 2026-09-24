BEGIN;

CREATE TABLE player (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    username TEXT NOT NULL UNIQUE CHECK (username ~ '[^[:space:]]'),
    email TEXT NOT NULL UNIQUE CHECK (email ~ '[^[:space:]]')
);

CREATE TABLE game (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title TEXT NOT NULL CHECK (title ~ '[^[:space:]]'),
    release_date DATE NOT NULL
);

CREATE TABLE genre (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL UNIQUE CHECK (name ~ '[^[:space:]]')
);

CREATE TABLE game_genre (
    game_id BIGINT NOT NULL REFERENCES game (id),
    genre_id BIGINT NOT NULL REFERENCES genre (id),
    PRIMARY KEY (game_id, genre_id)
);

CREATE TABLE player_game (
    player_id BIGINT NOT NULL REFERENCES player (id),
    game_id BIGINT NOT NULL REFERENCES game (id),
    added_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    playtime_minutes INTEGER NOT NULL DEFAULT 0 CHECK (playtime_minutes >= 0),
    status TEXT NOT NULL DEFAULT 'not_started'
        CHECK (status IN ('not_started', 'playing', 'completed', 'abandoned')),
    PRIMARY KEY (player_id, game_id)
);

CREATE TABLE achievement (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    game_id BIGINT NOT NULL REFERENCES game (id),
    name TEXT NOT NULL CHECK (name ~ '[^[:space:]]'),
    description TEXT NOT NULL CHECK (description ~ '[^[:space:]]'),
    points INTEGER NOT NULL CHECK (points >= 0),
    UNIQUE (game_id, name),
    -- The composite key lets an unlock verify the achievement's game below.
    UNIQUE (id, game_id)
);

CREATE TABLE player_achievement (
    player_id BIGINT NOT NULL,
    achievement_id BIGINT NOT NULL,
    -- Storing the game also lets PostgreSQL enforce library membership without a trigger.
    game_id BIGINT NOT NULL,
    unlocked_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (player_id, achievement_id),
    FOREIGN KEY (achievement_id, game_id) REFERENCES achievement (id, game_id),
    FOREIGN KEY (player_id, game_id) REFERENCES player_game (player_id, game_id)
);

COMMIT;
