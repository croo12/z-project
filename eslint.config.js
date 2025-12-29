import js from "@eslint/js";
import globals from "globals";

export default [
  js.configs.recommended,
  {
    files: ["src/**/*.js"],
    languageOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
      globals: {
        ...globals.browser,
        window: "readonly"
      }
    },
    rules: {
      "no-unused-vars": "warn",
      "no-console": "warn"
    }
  }
];
