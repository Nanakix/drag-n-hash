# Drag-n-hash

A simple cross-platform GUI application to get hashes from a file dragged-and-dropped onto the application window. 

# Features
- [X] A Window
- [X] When drag-and-dropping a file on that window:
  - [X] Compute the MD5, SHA1, SHA256, CRC32 hashes of the dropped file
  - [X] Show file information (hashes, size in human-readable format)
  - [X] Copy to clipboard
- [X] A sober but clean UI

# Possible Evolutions
- When dropping a file on the binary icon while in file explorer, directly copy data to clipboard while GUI window is spawning.
- Add a Column on the left-side with the desired information to select for computing/parsing.
- Lots of auto-parsing possibilities (magic numbers, ROM headers, ...)

# Resources

Here is a list of sources I browsed to make this project come to life, I would like to thank all of the people involved in these:
- The [Iced](https://iced.rs/) project, for the GUI
- [Alican](https://alican.codes/rust-github-actions) for the CI workflow
