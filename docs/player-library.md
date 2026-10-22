# Player library

The player library is this project's equivalent of the assignment's order detail view. It displays a selected player and their related games.

Run `cargo run -- library <player_id>` with `DATABASE_URL` configured, or run `cargo run -- gui`, choose **Library**, enter the player ID, and click **Run**.

The result contains the player's ID, username, and email, followed by their games with status, playtime in minutes, and date added. Each game lists all its achievements, including locked ones, with the achievement ID, name, description, points, and the selected player's Locked or Unlocked status. Games without achievements still appear. Games and achievements are ordered by their IDs. An existing player with no games still appears with an empty-library message. A missing player produces an error.

PostgreSQL loads the complete view with one parameterized SELECT and four LEFT JOINs:

```sql
SELECT p.id AS player_id, p.username, p.email,
       pg.game_id, g.title, pg.status, pg.playtime_minutes, pg.added_at::text,
       a.id, a.name, a.description, a.points,
       pa.achievement_id IS NOT NULL AS unlocked
FROM player p
LEFT JOIN player_game pg ON pg.player_id = p.id
LEFT JOIN game g ON g.id = pg.game_id
LEFT JOIN achievement a ON a.game_id = g.id
LEFT JOIN player_achievement pa ON pa.achievement_id = a.id AND pa.player_id = p.id
WHERE p.id = $1
ORDER BY pg.game_id, a.id;
```

Starting from `player` preserves a row for an empty library. No rows means the player does not exist. Joining all game achievements before the player's unlock records keeps locked achievements in the result. The player condition on the unlock join prevents another player's unlocks from affecting the displayed status. The mapper groups the returned rows into one player and their games, collecting each game's achievements without duplicating the game or creating entries for absent LEFT JOIN relations.

Data, domain, service, and presentation each have an explicit library model and mapper. Both the CLI and GUI use the same service operation and result formatting. JSON storage constructs the same view from its existing records without changing the file format.
