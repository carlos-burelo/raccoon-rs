# Raccoon Standard Library

The Raccoon standard library provides essential functionality for common operations.

## Modules

### Core Data Structures

- **array.rcc** - Array operations
  - `length<T>(arr: T[]): int`
  - `push<T>(arr: T[], item: T): void`
  - `pop<T>(arr: T[]): T`
  - `map<T, R>(arr: T[], callback: (T, int) => R): R[]`
  - `filter<T>(arr: T[], predicate: (T, int) => bool): T[]`
  - `reduce<T, R>(arr: T[], callback: (R, T, int) => R, initial: R): R`

- **string.rcc** - String operations (as String class)
  - `String.length(s: str): int`
  - `String.upper(s: str): str`
  - `String.lower(s: str): str`
  - `String.trim(s: str): str`
  - `String.split(s: str, delimiter: str): str[]`
  - `String.replace(s: str, from: str, to: str): str`

### Serialization

- **json.rcc** - JSON parsing and stringification
  - `parse(json: str): any`
  - `stringify(value: any): str`
  - `stringifyPretty(value: any, indent: int): str`
  - `tryParse(json: str): any | null`
  - `tryStringify(value: any): str | null`

### Math

- **math.rcc** - Mathematical functions and constants (as Math class)
  - Constants: `PI`, `E`, `TAU`, `SQRT2`, `LN2`, `LN10`, `EPSILON`
  - Functions: `abs()`, `min()`, `max()`, `sqrt()`, `pow()`, `sin()`, `cos()`, `tan()`
  - Utilities: `factorial()`, `gcd()`, `lcm()`, `toRadians()`, `toDegrees()`

### I/O

- **io.rcc** - File and directory operations
  - `readFile(path: str): str`
  - `writeFile(path: str, content: str): void`
  - `appendFile(path: str, content: str): void`
  - `fileExists(path: str): bool`
  - `deleteFile(path: str): void`
  - `readDir(path: str): str[]`
  - `createDir(path: str): void`
  - `input(prompt: str): str` - Read from stdin

### Time

- **time.rcc** - Time utilities
  - `now(): int` - Current timestamp in milliseconds
  - `sleep(ms: int): void` - Sleep for N milliseconds
  - `format(timestamp: int, format: str): str` - Format timestamp
  - `Timer` class - Measure execution time

### Examples

- **decorators_demo.rcc** - Demonstrates the decorator system
  - Shows usage of `@pure()`, `@inline()`, `@cache()`, `@deprecated()`

## Using the Standard Library

```raccoon
// Import specific functions
import { map, filter } from "std:array";
import { parse, stringify } from "std:json";
import Math from "std:math";

// Use in code
let numbers = [1, 2, 3, 4, 5];
let doubled = map(numbers, (n) => n * 2);
let sum = Math.abs(-42);
let json = stringify({x: 1, y: 2});
```

## Decorator System

All stdlib functions support Raccoon's decorator system for metadata:

```raccoon
@pure()           // No side effects, safe for optimization
@inline()         // Suggest function inlining
@cache(5000)      // Cache results for 5 seconds
@deprecated(msg)  // Mark as deprecated
```

See `decorators_demo.rcc` for examples.
