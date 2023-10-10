import basicSsl from '@vitejs/plugin-basic-ssl';
import autoImport from 'unplugin-auto-import/vite';
import type { PluginOption } from 'vite';
import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';

import { resolve } from 'node:path';

const root = process.cwd();

/** Path resolver. */
const pathResolve = (pathname: string) => {
    return resolve(root, '.', pathname);
};

export default defineConfig(({ mode }) => {
    const isDev = mode === 'development';

    const plugins = [
        solidPlugin(),
        autoImport({
            dts: `${pathResolve('types/auto-imports.d.ts')}/`,
            eslintrc: {
                enabled: true,
            },
            imports: [
                'solid-js',
                {
                    from: 'solid-js',
                    imports: ['Show', 'Match', 'For', 'createUniqueId'],
                },
                {
                    from: 'solid-js',
                    imports: ['Component', 'JSXElement', 'JSX', 'Accessor', 'Signal', 'ParentProps', 'Setter'],
                    type: true,
                },
            ],
        }) as PluginOption,
    ];

    if (isDev) {
        plugins.push(basicSsl());
    }

    return {
        build: {
            chunkSizeWarningLimit: 1500,
            emptyOutDir: true,
            minify: true,
            outDir: `${pathResolve('build/dist')}/`,
            target: 'esnext',
        },
        plugins,
        resolve: {
            alias: [
                {
                    find: /\/@\//,
                    replacement: `${pathResolve('src')}/`,
                },
                {
                    find: /\/#\//,
                    replacement: `${pathResolve('types')}/`,
                },
                {
                    find: /@\//,
                    replacement: `${pathResolve('src')}/`,
                },
                {
                    find: /#\//,
                    replacement: `${pathResolve('types')}/`,
                },
            ],
        },
        server: {
            port: 3000,
            proxy: {
                '/api': {
                    channgeOrigin: true,
                    secure: false,
                    target: 'http://0.0.0.0:8088',
                    ws: true,
                },
            },
        },
    };
});
