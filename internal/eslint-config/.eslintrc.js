/**
 * Eslint configuration.
 * @module internal/eslint-config/.eslintrc.js
 * @see module:internal/eslint-config
 */

module.exports = {
    extends: ["@internal/eslint-config"],
    ignorePatterns: [".eslintrc.js", ".turbo", "/build/**", "/dist/**", "/node_modules/**"],
    parserOptions: {
        project: ["tsconfig.json"],
        tsconfigRootDir: __dirname,
        sourceType: "module",
    },
    root: true,
    overrides: [
        {
            files: ["src/*.ts", "*.config.ts"],
            rules: {
                "@typescript-eslint/indent": "off",
                "@typescript-eslint/quotes": ["error", "double"],
                "react/jsx-filename-extension": "off",
                "import/no-default-export": "off",
                "quotes": "off",
            },
        },
    ],
};
