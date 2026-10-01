# Game library manager

- This is a school project. Favor small, easy-to-explain implementations and keep verification proportional to the assignment's scope.
- Keep this project a small synchronous Rust command-line application. Use plain SQL for the PostgreSQL backend and Docker Compose for local database setup. Avoid an ORM, server framework, or async application runtime unless requirements justify one.
- Model game genres as a many-to-many relationship. Track library entries and achievement unlocks per player.
- Enforce data relationships and uniqueness in each storage backend, including that an unlocked achievement belongs to a game in the player's library. Preserve PostgreSQL's database constraints.
- Give each domain entity its own Rust type and source file. Keep command dispatch separate from persistence, with storage operations behind a backend-independent interface. Avoid additional repository or service layers without a concrete need.

## File storage

- Support a single JSON file as an alternative to PostgreSQL. Create an empty library automatically when the selected file does not exist; reject invalid existing files without overwriting them.
- Keep JSON storage to basic sequential file loading and saving. Do not add locking, file versioning, or filesystem edge-case infrastructure unless requested.
