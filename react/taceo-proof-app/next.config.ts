import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  reactStrictMode: true,
  experimental: {
    externalDir: true,
  },
  // Workaround source: https://github.com/vercel/next.js/issues/29362#issuecomment-971377869
  webpack(config, { isServer, dev }) {
    config.experiments = {
      asyncWebAssembly: true,
      layers: true,
    };

    if (!dev && isServer) {
      config.output.webassemblyModuleFilename = 'chunks/[id].wasm';
      config.plugins.push(new WasmChunksFixPlugin());
    }

    return config;
  },
  output: 'standalone',
};

export default nextConfig;

class WasmChunksFixPlugin {
  apply(compiler: any) {
    compiler.hooks.thisCompilation.tap('WasmChunksFixPlugin', (compilation: any) => {
      compilation.hooks.processAssets.tap({ name: 'WasmChunksFixPlugin' }, (assets: any) =>
        Object.entries(assets).forEach(([pathname, source]) => {
          if (!pathname.match(/\.wasm$/)) return;
          compilation.deleteAsset(pathname);

          const name = pathname.split('/')[1];
          const info = compilation.assetsInfo.get(pathname);
          compilation.emitAsset(name, source, info);
        }),
      );
    });
  }
}
