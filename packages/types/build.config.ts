/**
 * Build config.
 * @module internal/types/build-config.ts
 */
import { defineBuildConfig } from "unbuild";

export default defineBuildConfig({
    clean: true,
    declaration: true,
    entries: ["src/index"],
    rollup: {
        emitCJS: true,
    },
});
