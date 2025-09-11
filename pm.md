TODO:

- Rust-ify existing functionality.
  - Write to csv file correctly. If possible, through a sqlite crate, if not
    then directly appending to the files. Will make modifying more difficult

  - Read from file correctly. Should be possible regardless of the state of
    writing, but need to transfer the reads.

- GUI side
  - [x] display text of any kind
    - `brew install sdl3 sdl3_ttf`
  - [x] display a timestamp of any kind
  - [x] debug menu view
    - can be toggled with f3
    - shows fps with ability to add more values
  - [ ] timer component
  - [ ] button component
    - a generic button
    - two states: pressed and unpressed
    - clicked, so mouse works
    - needs to cause side effects
  - [ ] input system such that components can handle inputs themselves

        - [ ] mouse directs to the correct component
        - [ ] keypresses direct to all components? just to the ones that subscribe? maybe a different mode?
        - [ ] maybe a filter helper function given a position / hitbox / something
              and have it be called if we want to slim down to certain things


    Font being committed to for the time being.

- https://ttfonts.net/font/17298_Futura.htm
