# Carrot


### How it works
```mermaid
flowchart TD
    I[Input]
    --> |Source text| PPP[Pre-preprocessor]
    --> |Escaped text| PP[Preprocessor]
    --> |Escaped text with whitespace removed| T[Tokeniser]
    --> |Tokens| F[Funcifier]
    --> |Function tree| L[Linker]
    --> |Linked tree| B[Codegen]
    B --> |Bytecode| VM[Run in CVM]
    B --> |Bytecode| C[Compile/transpile to other target]
    C --> Run
```