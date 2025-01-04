To use vendored sources, add this to your .cargo/config.toml for this project:

[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"

sqlx migrate add api_key

CREATE TABLE IF NOT EXISTS api_keys
(
    api_key BLOB PRIMARY KEY
);

sqlx migrate run