/**
 * Project eslint config for `solid-js`.
 * @module internal/eslint-config/solid
 */

export default {
    'extends': ['@internal/eslint-config', 'iivanovw7/typedoc', 'iivanovw7/feature-sliced', 'iivanovw7/solid'],
    overrides: [
        {
            files: ['*.css.ts'],
            rules: {
                '@typescript-eslint/no-magic-numbers': 'off',
            },
        },
    ],
    rules: {
        'import/extensions': [
            'error',
            'ignorePackages',
            {
                js: 'never',
                jsx: 'never',
                mjs: 'never',
                ts: 'never',
                tsx: 'never',
            },
        ],
        'import/no-unresolved': 'error',
        'no-param-reassign': 'off',
        'react/jsx-filename-extension': 'off',
        'solid/jsx-no-undef': [
            'error',
            {
                allowGlobals: true,
                autoImport: true,
                typescriptEnabled: false,
            },
        ],
    },
};
