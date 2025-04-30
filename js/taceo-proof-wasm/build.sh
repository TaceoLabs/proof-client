VERSION="0.0.0"

if [ "$#" -ne 1 ]; then
  echo "Usage: build.sh <target>  supported targets (node, browser)"
  exit 1
fi

if [ "$1" = "node" ]; then
  echo "building for node"
  wasm-pack build --target nodejs --scope taceo
  sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" ./pkg/package.json
  sed -i 's/"@taceo\/taceo-proof-wasm"/"@taceo\/proof-wasm"/' ./pkg/package.json
elif [ "$1" = "browser" ]; then
  echo "building for browser"
  wasm-pack build --target bundler --scope taceo
  sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" ./pkg/package.json
  sed -i 's/"@taceo\/taceo-proof-wasm"/"@taceo\/proof-wasm-browser"/' ./pkg/package.json
else
  echo "unsupported target"
  exit 1
fi
