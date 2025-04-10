VERSION="0.0.0"

if [ "$#" -ne 1 ]; then
  echo "Usage: build.sh <target>  supported targets (node, browser)"
  exit 1
fi

if [ "$1" = "node" ]; then
  echo "building for node"
  sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" package.json
  sed -i 's/"@taceo\/ppd-client-browser"/"@taceo\/ppd-client"/' package.json
  sed -i 's/"@taceo\/ppd-wasm-browser"/"@taceo\/ppd-wasm"/' package.json
  npm i && npm run build
elif [ "$1" = "browser" ]; then
  echo "building for browser"
  sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" package.json
  sed -i 's/"@taceo\/ppd-client"/"@taceo\/ppd-client-browser"/' package.json
  sed -i 's/"@taceo\/ppd-wasm"/"@taceo\/ppd-wasm-browser"/' package.json
  npm i && npm run build
else
  echo "unsupported target"
  exit 1
fi