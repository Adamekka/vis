# Adding a game to a library

`add-game-to-library` creates a new game, assigns existing genres, creates achievements, and adds the game to an existing player's library. Supply at least one genre and one achievement. Repeat `--genre` and `--achievement` to add more. Each achievement takes a name, description, and points.

After creating a player and genres with the standalone commands, use their IDs:

```sh
cargo run -- add-game-to-library 1 "New adventure" 2024-02-29 \
  --genre 1 --genre 2 \
  --achievement "First step" "Finish the tutorial" 10 \
  --achievement "Explorer" "Visit every area" 25
```

For JSON storage, put `--storage json --file /path/to/library.json` before the command. For PostgreSQL, configure `DATABASE_URL` as for the other commands.

In the native GUI, select **Add game to library**, enter the player ID, title, and release date, then fill in the genre and achievement rows. Use **Add genre**, **Add achievement**, and **Remove** to edit the lists. **Run** is enabled when all fields are filled and both lists contain at least one row. Success displays the created game and achievement IDs and the assigned genre IDs.

## Shared operations

The service validates the title, date, and achievement values before writing. It then performs these operations in order:

1. `AddGame` creates the game and returns its ID.
2. `TagGame` assigns each supplied genre to that ID.
3. `AddAchievement` creates each achievement for that ID.
4. `AddToLibrary` adds the game to the player with the same initial progress as the standalone command.

The composite operation calls the same service dispatch as the standalone commands. It does not add SQL statements or another storage implementation. Both PostgreSQL and JSON use their existing individual writes and constraints.

## Failure behavior

Each successful write remains saved. The workflow stops on the first failed write, and later steps do not run. There is no transaction spanning the workflow and no rollback of earlier writes.

For example, if the second achievement repeats the first one's name, the game, genre assignments, and first achievement remain saved. The second achievement is rejected, any subsequent achievements are skipped, and the game is not added to the player's library. The error identifies the failed step and lists the saved game, genre, and achievement IDs. A missing player fails at the final step and also leaves the earlier writes saved.

Continue with `tag-game`, `add-achievement`, and `add-to-library` using the saved game ID. Running the entire composite command again creates another game.

To demonstrate a partial failure, repeat an achievement name in the command above, then inspect `games`, `achievements <game_id>`, and `library <player_id>`. The same behavior applies to both storage backends.
