# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### To Do

- ToIR implementations:
  - Html colours (8, 16, 24 and 32-bit)
  - File path (load bytes from file, implement as a command line option)  
- FromIR implementations:
  - Colours (8, 16, 24 and 32-bit)
  - Disassembly (16, 32 and 64-bit)
  - File path (save bytes from file, implement as a command line option, with optional prompt to confirm the save)
- Add command line options for
  - Endianness
      - Add global functionality to edit endianness. Also allow types to specify their own custom endianness
      - Types:    
        - Default (some types use big endian, some use little)
          - Allowing a default is really important, since some types show their LSB on the left (escape sequences, byte lists, etc.) and some show their LSB on the right (numbers, IP addresses, etc.)
        - Opposite (opposite of default)
        - Little (always little endian)
        - Big (always big)
  - Hex case formatting
  - ascii table
- Ignore whitespace with certain types (numbers)
- Provide a variant for FromIr Floats showing mantissa, exponent and signs

### Unfinished Ideas
None

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