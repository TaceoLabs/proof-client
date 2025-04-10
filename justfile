generate-js-api-client:
  (rm -r js/ppd-api-client || true) && docker run --rm \
    -u $(id -u ${USER}):$(id -g ${USER}) \
    -v ${PWD}:/local openapitools/openapi-generator-cli generate \
    -i /local/v1-openapi.json \
    -g typescript-fetch \
    -o /local/js/ppd-api-client \
    --additional-properties=supportsES6=true,npmName=ppd-api-client

generate-rust-api-client:
  (rm -r rust/ppd-api-client || true) && docker run --rm \
    -u $(id -u ${USER}):$(id -g ${USER}) \
    -v ${PWD}:/local openapitools/openapi-generator-cli generate \
    -i /local/v1-openapi.json \
    -g rust \
    -o /local/rust/ppd-api-client \
    --additional-properties=packageName=ppd-api-client && cd rust/ppd-api-client && cargo fmt && \
  sed -i '1i #![allow(clippy::all)]\n#![allow(unused_mut)]\n#![allow(unused_variables)]\n' src/lib.rs && \
  sed -i 's/"multipart"/"multipart", "stream"/' Cargo.toml && \
  sed -i -E 's/(input_party[0-2]): std::path::PathBuf,/\1: impl Into<reqwest::Body>,/' src/apis/job_api.rs && \
  sed -i 's/let mut local_var_form = reqwest::multipart::Form::new();/let local_var_form = reqwest::multipart::Form::new().part("input_party0", reqwest::multipart::Part::stream(input_party0)).part("input_party1", reqwest::multipart::Part::stream(input_party1)).part("input_party2", reqwest::multipart::Part::stream(input_party2));/' src/apis/job_api.rs && \
  cargo fmt
