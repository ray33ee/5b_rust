# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### To Do
- ToIR implementations:
  - File path (load bytes from file, implement as a command line option)  
- FromIR implementations:
  - Disassembly (16, 32 and 64-bit)
  - File path (save bytes from file, implement as a command line option, with optional prompt to confirm the save)
- Add command line options for
  - Hex case formatting
  - ascii table
  - Forcing little/big endianness (instead of prompting user) when converting to bytes
- Ignore whitespace with certain types (numbers)
- Include some example usage in readme (escaping backslashes for regexes, identifying invalid unicode characters, basic conversions and some esoteric conversions)
- Pipe input to and from 5b
- Get away from `colour` and use the `ansi_term` crate instead
- Add precompiled binary to sourceforge and add link to readme
- Upgrade readme (build instructions, links to precompiled binaries, example usage, explination of  benefits of 5b)

- Big three to focus on:
  - Change colouring to `ansi_term`
  - Sort out colouring, formatting and presentation
  - Include command options
  - Improve readme

### Unfinished Ideas
- Not sure about disassembly. But if we want to use it, [this](https://crates.io/crates/iced-x86) seems to be the solution 

## [0.1.5] - 2021-07-03
### Added
- When the selected type is `Dual` endianness, we ask the user which (little or big) endianness they would like to interpret the data as
- `FixedInt` now converts via `Base2_16` so we accept base 2, 8, 10 and 16 for the primitive integers 
  - This only works when numbers are prefixed (0x, 0o and 0b) otherwise it is treated as decimal.
  - This means that non-prefixed binary numbers will assumed to be decimal by `FixedInt` and non-prefixed octal or hex numbers will be ignored by `FixedInt`
  - They will still be picked up by `Base2_16` though
- `FixedFloat` now displays floats using scientific notation and SI unit notation
- `Endian` trait representing `Endianness` for types 

### Fixed
- We now check for invalid dates (out of range usually) and returns an error string instead of the formatted date
- No longer panics when an empty string is read via `read_without_newline`

### Removed
- `Hash` object removed as this is a bytes to bytes object (at the moment we only deal with string to bytes or bytes to string objects)

## [0.1.4] - 2021-06-26
### Added
- Unicode names which allows the user to 
  - Convert a unicode name into a character
  - Convert a string into a list of unicode names
- Added `common::Colour` to represent colours of various bit depths. Supplies some functions to convert to 24-bit colour  
- `FromIR` implementation for colours supporting
  - 8, 16, 24 and 32-bit colours
  - 8-bit colour, greyscale and terminal formats  
  - RGB and HSL values for 24-bit
- `ToIR` implementation of colours supporting html colour format (#RRGGBB)

### Changed
- `FromIR::encode` now returns `ANSIGenericString<str>` which allows styling of terminal strings
- Showing dual endianness now only colours the brackets magenta to allow styled text

## [0.1.3] - 2021-06-12
### Added
- Mantissa/Exponent form for `FixedFloat` added
- Endianness
  - `ToIr`: Always store data as little endian
  - `FromIr`: Two options, display the default endianness (chosen by `FromIr::encode`) or both little and big if no default can be chosen (primitive integers and float for example)
  - Endianness is implemented by
    - Adding a function to `FromIR` that identifies if a type has default or dual endianness
    - The main control flow by displaying a variants on their own (default) or two values for each variant, one little and another big endian (dual)

### Fixed
- Escape sequence code now accepts \fff as a valid python sequence as well as a C sequence

## [0.1.2] - 2021-06-05
### Added
- Byte list now shows the number of bytes 
- Variants for C and Python escape sequences
- Python style Named unicode characters added to the escape sequence type (\N)
- Unicode escape characters (\U and \u)

### Changes
- FromIT typo corrected to FromIR
- EscapedString renamed to EscapeSequence

### Fixed
- When checking for prefixes ('0x', '0o', etc.) we now test for the size before we take a slice

## [0.1.1] - 2021-06-02
### Added
- Base conversion has been shortened to base 2, 8, 10 and 16
- Endianness of Ipv4 and Ipv6 is fixed
- Colour added to terminal output
- Prefixes 0x, 0b and 0o are now supported
- URL decoding and encoding added
- `FromIT::variants` and `ToIR::identify` return an Option<Vec<_>> and return None if no variants match to avoid unnecessary allocations 
- Warnings cleared

## [0.1.0] - 2021-06-01
### Added
- Readme
- This changelog
- License
- Basic command line interface
- Escaped string
- UUID FromIR
- Removed Hash object