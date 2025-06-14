# rustify home evaluator

a simple CLI property manager written in Rust. this project was built to practice the fundamentals covered in the first nine chapters of *the rust programming language*.


## Features

- add new properties with:
  - total square meters
  - number of bedrooms
  - number of bathrooms
  - garden size (m²)
  - house type (apartment / detached / semi-detached / townhouse)
- list all properties with detailed information
- evaluate each property:
  - spacious vs. cozy
  - has garden vs. no garden
- filter by number of bedrooms (≥ 3)
- delete a property by its index


## prerequisites

- [rust and cargo](https://www.rust-lang.org/tools/install) installed


## installation

```bash
git clone https://github.com/mikel0x/rustify_home_eval.git
cd rustify_home_eval
