# Human Verification Plan

## Local setup

- Run `cargo run`
- Confirm the banner appears
- Confirm the prompt `> ` appears

## Core demo walkthrough

- Enter `X*Y`
- Confirm the output includes:
  - `parsed: XY`
  - at least one `step` line
  - `result: iZ`

- Enter `[X,Y]`
- Confirm the output includes:
  - `parsed: [X,Y]`
  - at least one `step` line
  - `result: 2iZ`

- Enter `(X+Y)*(X+Y)`
- Confirm the output includes:
  - `parsed: (X + Y)(X + Y)`
  - at least one `step` line
  - `result: 2I`

## Help and examples

- Enter `help`
- Confirm the command list is shown

- Enter `examples`
- Confirm the three demo expressions are shown

## Error handling

- Enter `H`
- Confirm an error message appears for an unknown symbol

- Enter `(X+Y`
- Confirm a parse-related error appears

## Demo readability

- Confirm outputs are easy to read in a normal terminal window
- Confirm `parsed`, `step`, and `result` are visually distinguishable
- Confirm the three demo commands can be presented without manual editing

## Recovery

- If the session gets into a bad state, enter `exit` and restart with `cargo run`
