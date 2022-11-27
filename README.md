# Carrot


### How it works
```mermaid
flowchart TD
    I[Input]
    --> |Source text| PPP[Pre-preprocessor]
    --> |Escaped text| PP[Preprocessor]
    --> |Escaped text with whitespace removed| T[Tokeniser]
    --> |Tokens| F[Conver to functions]
    --> |Function tree| B[Convert to bytecode]
    B --> |Bytecode| VM[Run in CVM]
    B --> |Bytecode| C[Compile/transpile to other target]
    C --> Run
```