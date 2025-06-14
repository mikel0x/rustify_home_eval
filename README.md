# Rustify Home Evaluator

A simple CLI property manager written in Rust. This project was built to practice the fundamentals covered in the first nine chapters of *The Rust Programming Language*.

## Features

- **Add** new properties with:
  - Total square meters
  - Number of bedrooms
  - Number of bathrooms
  - Garden size (m²)
  - House type (Apartment / Detached / Semi-Detached / Townhouse)
- **List** all properties with detailed information
- **Evaluate** each property:
  - Spacious vs. Cozy
  - Has garden vs. No garden
- **Filter** by number of bedrooms (≥ 3)
- **Delete** a property by its index

## Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install) installed

## Installation

```bash
git clone https://github.com/mikel0x/rustify_home_eval.git
cd rustify_home_eval
