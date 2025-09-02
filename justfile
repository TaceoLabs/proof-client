generate-js-api-client:
  (rm -r js/taceo-proof-api-client || true) && docker run --rm \
    -u $(id -u ${USER}):$(id -g ${USER}) \
    -v ${PWD}:/local:z openapitools/openapi-generator-cli:v7.12.0 generate \
    -i /local/v1-openapi.json \
    -g typescript-fetch \
    -o /local/js/taceo-proof-api-client \
    --additional-properties=supportsES6=true,npmName=@taceo/proof-api-client,npmVersion=2.0.0 && cd js/taceo-proof-api-client && npm i

generate-rust-api-client:
  (rm -r rust/taceo-proof-api-client || true) && docker run --rm \
    -u $(id -u ${USER}):$(id -g ${USER}) \
    -v ${PWD}:/local:z openapitools/openapi-generator-cli:v7.12.0 generate \
    -i /local/v1-openapi.json \
    -g rust \
    -o /local/rust/taceo-proof-api-client \
    --additional-properties=packageName=taceo-proof-api-client && cd rust/taceo-proof-api-client && cargo fmt && \
  sed -i '1i #![allow(clippy::all)]\n#![allow(unused_mut)]\n#![allow(unused_variables)]\n' src/lib.rs && \
  sed -i 's/"multipart"/"multipart", "stream", "cookies"/' Cargo.toml && \
  sed -i -E 's/(witness[0-2]): std::path::PathBuf,/\1: impl Into<reqwest::Body>,/' src/apis/job_api.rs && \
  sed -i -E 's/(inputs[0-2]): std::path::PathBuf,/\1: impl Into<reqwest::Body>,/' src/apis/job_api.rs && \
  sed -i 's|// TODO: support file upload for '\''witness0'\'' parameter|multipart_form = multipart_form.part("witness0", reqwest::multipart::Part::stream(p_witness0));|' src/apis/job_api.rs && \
  sed -i 's|// TODO: support file upload for '\''witness1'\'' parameter|multipart_form = multipart_form.part("witness1", reqwest::multipart::Part::stream(p_witness1));|' src/apis/job_api.rs && \
  sed -i 's|// TODO: support file upload for '\''witness2'\'' parameter|multipart_form = multipart_form.part("witness2", reqwest::multipart::Part::stream(p_witness2));|' src/apis/job_api.rs && \
  sed -i 's|// TODO: support file upload for '\''inputs0'\'' parameter|multipart_form = multipart_form.part("inputs0", reqwest::multipart::Part::stream(p_inputs0));|' src/apis/job_api.rs && \
  sed -i 's|// TODO: support file upload for '\''inputs1'\'' parameter|multipart_form = multipart_form.part("inputs1", reqwest::multipart::Part::stream(p_inputs1));|' src/apis/job_api.rs && \
  sed -i 's|// TODO: support file upload for '\''inputs2'\'' parameter|multipart_form = multipart_form.part("inputs2", reqwest::multipart::Part::stream(p_inputs2));|' src/apis/job_api.rs && \
  sed -i 's/models::models/models/' src/apis/blueprint_api.rs && \
  sed -i 's/models::models/models/' src/apis/job_api.rs && \
  sed -i 's/models::models/models/' src/apis/admin_api.rs && \
  sed -i 's/file: std::path::PathBuf,/file: impl Into<reqwest::Body>,/' src/apis/blueprint_api.rs && \
  sed -i 's/let mut multipart_form = reqwest::multipart::Form::new();/let multipart_form = reqwest::multipart::Form::new().part("file", reqwest::multipart::Part::stream(p_file));/' src/apis/blueprint_api.rs && \
  sed -i '/fn default() -> Self {/,/}/c\    fn default() -> Self {\n use reqwest::ClientBuilder;\n       let client = ClientBuilder::new()\n            .cookie_store(true)\n            .build()\n            .expect("can build reqwest client");\n        Configuration {\n            base_path: "http://localhost".to_owned(),\n            user_agent: Some("OpenAPI-Generator/1.0/rust".to_owned()),\n            client,\n            basic_auth: None,\n            oauth_access_token: None,\n            bearer_access_token: None,\n            api_key: None,\n        }\n    ' src/apis/configuration.rs && \
  cargo fmt

