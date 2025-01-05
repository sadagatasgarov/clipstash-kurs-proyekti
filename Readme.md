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


cargo run --bin clipclient -- --api-key
http://localhost:8000/api/clip/key

   >> No matching routes for GET /favicon.ico image/avif.
   >> Responding with registered (not_found) 404 catcher.
Api Key: 9gGXBE9QcA7CustTyakZRA==



cargo run --bin clipclient -- --api-key 9gGXBE9QcA7CustTyakZRA==
