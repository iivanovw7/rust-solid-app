/**
 * Eslint configuration.
 * @module src/.eslintrc.js
 * @see module:internal/eslint-config/solid
 */
module.exports = {
    extends: ["@internal/eslint-config/solid", "./../.eslintrc-auto-import.json"],
    rules: {
        "@typescript-eslint/no-redeclare": "off",
        "arrow-body-style": "off",
        "import/no-absolute-path": "off",
    },
    parserOptions: {
        project: ["../tsconfig.json"],
        tsconfigRootDir: __dirname,
    },
    ignorePatterns: [".eslintrc.js"],
    root: true,
    overrides: [
        {
            files: ["index.tsx"],
            rules: {
                "import/no-unresolved": "off",
            },
        },
    ],
    settings: {
        "import/resolver": {
            typescript: {},
        },
    },
};
