import adapter from "@sveltejs/adapter-node";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { sveltekit } from "@sveltejs/kit/vite";
import { svelteTesting } from "@testing-library/svelte/vite";
import { defineConfig } from "vitest/config";

export default defineConfig({
    // `svelteTesting()` selects Svelte's browser/client resolve condition so
    // components mount client-side under jsdom (rather than SSR), and wires
    // testing-library auto-cleanup. It only affects the vitest run.
    plugins: [
        sveltekit({
            preprocess: vitePreprocess(),
            adapter: adapter(),
            alias: { $lib: "src/lib" },
            csp: {
                // Nonces for dynamically rendered pages, hashes for prerendered
                // ones; SvelteKit adds them to script-src and style-src.
                mode: "auto",
                directives: {
                    "default-src": ["self"],
                    "script-src": ["self"],
                    "style-src": ["self"],
                    // Style ATTRIBUTES only. The SVAR grid, Gantt, calendar and
                    // Kanban set a computed `style="…"` on rows, cells and bars
                    // (virtual-scroll offsets, row heights, bar positions), and
                    // SvelteKit's route announcer carries one too; none of those
                    // can be hashed or given a nonce. Scripts and <style>
                    // elements stay strict ('self' + kit nonces/hashes), so
                    // this does not let injected markup run code or load CSS.
                    "style-src-attr": ["unsafe-inline"],
                    "img-src": ["self", "data:"],
                    "font-src": ["self"],
                    "connect-src": ["self"],
                    "object-src": ["none"],
                    "base-uri": ["self"],
                    "form-action": ["self"],
                    "frame-ancestors": ["none"]
                }
            }
        }),
        svelteTesting()
    ],
    server: { port: 5173, strictPort: false },
    test: {
        include: ["tests/unit/**/*.{test,spec}.{ts,js}"],
        environment: "jsdom",
        globals: true,
    },
});
