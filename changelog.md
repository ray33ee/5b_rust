# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### To Do

- ToIR implementations:
  - Html colours
  - File path (load bytes from file)
- FromIR implementations:
  - Colours
  - Disassembly (16, 32 and 64-bit)
  - File path (save bytes from file)
- Add command line options for
  - Endianness
  - Hex case formatting
  - ascii table
- Ignore whitespace with certain types (numbers)
- Provide a mantissa/exponent variant for FromIr Floats
- Convert from Ipv4 to Ipv6 address

### Unfinished Ideas
None

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