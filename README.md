# MyCompiler (UESTC-编译原理)

This project is a compiler developed as part of the Compilation Principles course at UESTC. It implements fundamental stages of a compiler, including preprocessing, lexical analysis, and parsing, for a simplified language.

## Project Structure and Components

The compiler is organized into several modules:

*   **`src/prep.rs`**: The Preprocessor handles initial processing of the source code.
*   **`src/lex.rs`**: The Lexer performs lexical analysis, breaking the source code into tokens and handling lexical errors. It includes tables for keywords, identifiers, and literals, and implements a simple state machine for token recognition.
*   **`src/parse.rs`**: The Parser implements an LL(1) grammar using a recursive descent approach to perform syntax analysis and report syntax errors.
*   **`src/env.rs`**: Manages the environment and symbol tables, handling variable and procedure declarations and scope management.
*   **`src/main.rs`**: The main entry point that orchestrates the compiler stages.

Diagrams and screenshots related to the project can be found in the `assets` folder.

## Future Work

Based on the `doc/未来工作.md`, the following areas are planned for future development:

*   Error recovery mechanisms.
*   Improved handling of procedure names for variable declarations.
*   Enhanced existence checks for factors during parsing.
*   Further checks on the scope of parameters in function declarations.
*   Refinement of file stream handling.
*   Implementation of a command-line software architecture.
*   Development of a table-driven DFA for lexical analysis.
*   Input optimization.

## Status

This project is currently under active maintenance and development.
