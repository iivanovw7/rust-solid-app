/**
 * Build config.
 * @module internal/eslint-config/build-config.ts
 */

import { defineBuildConfig } from 'unbuild';

export default defineBuildConfig({
    clean: true,
    declaration: true,
    entries: ['src/index', 'src/solid'],
    rollup: {
        emitCJS: true,
    },
});
