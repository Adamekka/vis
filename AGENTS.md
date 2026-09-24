# Game library manager

- Keep this project a small synchronous Rust command-line application using plain SQL and PostgreSQL in Docker Compose. Avoid an ORM, server framework, or async application runtime unless requirements justify one.
- Model game genres as a many-to-many relationship. Track library entries and achievement unlocks per player.
- Enforce data relationships and uniqueness in PostgreSQL, including that an unlocked achievement belongs to a game in the player's library.
- Give each domain entity its own Rust type and source file, keeping its database operations with the type. Keep command dispatch separate and avoid repository or service layers without a concrete need.
