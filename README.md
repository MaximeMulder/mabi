## Architecture

| Index | Name  | Next | Pop       | Push |
|-------|-------|------|-----------|------|
| 0     | NOP   | -    | -         | -    |
| 1     | LOAD  | val  | -         | val  |
| 2     | READ  | -    | addr      | val  |
| 3     | WRITE | -    | addr, val | -    |
| 4     | AND   | -    | a, b      | res  |
| 5     | OR    | -    | a, b      | res  |
| 6     | XOR   | -    | a, b      | res  |
| 7     | NOT   | -    | a         | res  |
| 8     | ADD   | -    | a, b      | res  |
| 9     | SUB   | -    | a, b      | res  |
| 10    | MUL   | -    | a, b      | res  |
| 11    | DIV   | -    | a, b      | res  |
| 12    | REM   | -    | a, b      | res  |
| 13    | JUMP  | -    | addr      | -    |
| 14    | IF    | -    | addr, val | -    |
| 15    | PRINT | -    | val       | -    |
| 16    | END   | -    | -         | -    |
