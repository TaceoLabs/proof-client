generate-js-api-client:
  (rm -r js/ppd-api-client || true) && docker run --rm \
    -u $(id -u ${USER}):$(id -g ${USER}) \
    -v ${PWD}:/local openapitools/openapi-generator-cli:v7.12.0 generate \
    -i /local/v1-openapi.json \
    -g typescript-fetch \
    -o /local/js/ppd-api-client \
    --additional-properties=supportsES6=true,npmName=ppd-api-client

generate-rust-api-client:
  (rm -r rust/ppd-api-client || true) && docker run --rm \
    -u $(id -u ${USER}):$(id -g ${USER}) \
    -v ${PWD}:/local openapitools/openapi-generator-cli:v7.12.0 generate \
    -i /local/v1-openapi.json \
    -g rust \
    -o /local/rust/ppd-api-client \
    --additional-properties=packageName=ppd-api-client && cd rust/ppd-api-client && cargo fmt && \
  sed -i '1i #![allow(clippy::all)]\n#![allow(unused_mut)]\n#![allow(unused_variables)]\n' src/lib.rs && \
  sed -i 's/"multipart"/"multipart", "stream", "cookies"/' Cargo.toml && \
  sed -i -E 's/(input_party[0-2]): std::path::PathBuf,/\1: impl Into<reqwest::Body>,/' src/apis/job_api.rs && \
  sed -i 's/let mut multipart_form = reqwest::multipart::Form::new();/let multipart_form = reqwest::multipart::Form::new().part("input_party0", reqwest::multipart::Part::stream(p_input_party0)).part("input_party1", reqwest::multipart::Part::stream(p_input_party1)).part("input_party2", reqwest::multipart::Part::stream(p_input_party2));/' src/apis/job_api.rs && \
  sed -i 's/file: std::path::PathBuf,/file: impl Into<reqwest::Body>,/' src/apis/blueprint_api.rs && \
  sed -i 's/let mut multipart_form = reqwest::multipart::Form::new();/let multipart_form = reqwest::multipart::Form::new().part("file", reqwest::multipart::Part::stream(p_file));/' src/apis/blueprint_api.rs && \
  sed -i '/fn default() -> Self {/,/}/c\    fn default() -> Self {\n use reqwest::ClientBuilder;\n       let client = ClientBuilder::new()\n            .cookie_store(true)\n            .build()\n            .expect("can build reqwest client");\n        Configuration {\n            base_path: "http://localhost".to_owned(),\n            user_agent: Some("OpenAPI-Generator/1.0/rust".to_owned()),\n            client,\n            basic_auth: None,\n            oauth_access_token: None,\n            bearer_access_token: None,\n            api_key: None,\n        }\n    ' src/apis/configuration.rs && \
  cargo fmt

