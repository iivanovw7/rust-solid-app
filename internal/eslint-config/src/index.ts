/**
 * Base project eslint config.
 * @module internal/eslint-config
 */
export default {
    env: {
        browser: true,
        es6: true,
        node: true,
    },
    'extends': [
        'turbo',
        'iivanovw7',
        'iivanovw7/typescript',
        'iivanovw7/order',
    ],
    globals: { defineOptions: 'readonly' },
    root: true,
    rules: {
        'arrow-body-style': 'off',
        'import/extensions': 'off',
        'import/no-unresolved': 'off',
        'node/no-missing-import': 'off',
        'node/no-unpublished-import': 'off',
        'react/jsx-filename-extension': 'off',
    },
};
