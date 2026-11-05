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

The composite operation calls the same service dispatch as the standalone commands inside one storage transaction. PostgreSQL uses a transaction on the existing connection and reuses the same SQL statements and constraints. JSON stages the individual changes in memory, validates them with the existing constraints, and saves the complete snapshot once. It writes a temporary file beside the library and replaces the library file only after saving succeeds.

## Failure behavior

The workflow is all-or-nothing on both backends. It stops on the first failed step and discards all changes made by the command. Existing players, games, genre assignments, achievements, and library entries stay unchanged. Success is reported only after the transaction commits.

For example, if the second achievement repeats the first one's name, the new game, genre assignments, and first achievement are all discarded. The error identifies the failed step. A missing player fails at the final step and also discards the earlier changes.

Correct the input and run the complete command again. PostgreSQL may leave gaps in generated IDs after a rollback because sequences are not rolled back.

To demonstrate rollback, repeat an achievement name in the command above, then inspect `all-items`. No part of the rejected game setup should appear. The same behavior applies to both storage backends.
