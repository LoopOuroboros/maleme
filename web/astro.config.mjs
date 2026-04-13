import { defineConfig, sessionDrivers } from "astro/config";
import cloudflare from "@astrojs/cloudflare";

export default defineConfig({
  output: "server",
  adapter: cloudflare({
    remoteBindings: false,
  }),
  image: {
    remotePatterns: [
      { protocol: "https", hostname: "avatars.githubusercontent.com" },
      { protocol: "https", hostname: "github.githubassets.com" },
    ],
  },
  session: {
    driver: sessionDrivers.null(),
  },
  security: {
    checkOrigin: false,
  },
});
