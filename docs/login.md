# Sign-in and roles

The app uses a deliberately naive login: a fixed list of demo accounts with plaintext passwords.

| Username | Password | Role |
| --- | --- | --- |
| `admin` | `admin` | Admin |
| `alex` | `alex` | Player 1 |
| `sam` | `sam` | Player 2 |

Commands sign in with `--user` and `--password`, for example `vis --user alex --password alex players`. The window shows a sign-in form first and a **Sign out** button afterwards.

## Rules

- Admins can run every action.
- Players can list players, games, genres, achievements, and all items, and can view or change only their own library, progress, and unlocked achievements. They cannot add players, games, genres, or achievements, or assign genres.
- `players` (and the players section of `all-items`) shows every email to admins. Players see only their own email; other emails show as `(hidden)`.

## Where the checks live

- **Presentation** requires a signed-in user before any action runs. The window lists only the actions the signed-in role may use.
- **Domain** verifies credentials and authorizes every operation, including the player-ownership check. It also decides which player emails are visible.
- **Service** receives the app-wide current user through its constructor. `main` creates one shared `CurrentUser` and injects it, so sign-in and sign-out are visible to every holder.
