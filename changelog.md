# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### To Do

- For a given input display all possible conversions. Allow the user to change the input to one of these conversions.
  - This allows an arbitrary number of conversions without having to keep entering data and identifying it
- ToIR implementations:
  - Python string formatted bytes list
  - GUID
  - Html colours
  - File path (load bytes from file)
- FromIR implementations:
  - Python string formatted bytes list
  - GUID
  - Colours
  - Disassembly (16, 32 and 64-bit)
  - File path (save bytes from file)
- Throw in a nice looking ascii table for good measure
- Modify the Base2_16 to allow variants for lower and upper case formatting
  - This will only affect `FromIR` since `ToIR` will accept a mix of upper and lower case
- Big/little endian-nes should be selected by reversing the IR
- Ignore whitespace with certain types (numbers)
- Provide a mantissa/exponent variant for FromIr Floats
- Find out why Ascii85 gives a 'attempt to multiply with overflow' panic when decoding
- Create an iterator over a string that converts escaped strings into bytes list
  - Create a function to convert bytes into escaped characters
- Improve appearance with tables and colours

### Unfinished Ideas
None

## [0.1.0] - 2021-05-05
### Added
- Readme
- This changelog
- License
- Basic command line interface
- Escaped string
- UUID FromIR
- Removed Hash object