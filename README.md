# rustcdp

Currently a work in a progress.

All interactions occur through the `Chrome` struct.

See `main.rs` for a working example.

Documentation will come after all stable domains are implemented.



----



### Project Structure:

- Browser:
  - Handles the spawning/tear down of the chrome executable.
  - Initialises and manages the websocket connection
- Domain
  - Holds the mappings to the chrome dev tool protocol, with each domain structured by types, methods and events. (https://chromedevtools.github.io/devtools-protocol)
- User Call Registry
  - Handles user interactions with the chrome instance.
  - Used to validate call success/failure
- State
    - *Still making considerations around the implementation of this element and its design*
    - Intention is to have a layer that can track the chrome browser's state, such as the current mouse location, the window size, etc through events. Will revisit this towards the end of the project. 