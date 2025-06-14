R U S T I F Y   H O M E   E V A L U A T O R

                      (c) 2025 by mikel0x
         built as a study project for chapters 1–9 of The Rust Book
            https://github.com/mikel0x/rustify_home_eval

             Licensed under MIT (see LICENSE file)

 THIS TOOL IS FOR EDUCATIONAL PURPOSES ONLY — NO ILLEGAL USE.

INTRODUCTION
------------
Rustify Home Evaluator is a simple command-line property manager written in Rust.
I built it to practice the fundamentals covered in the first nine chapters of
“The Rust Programming Language”.  It lets you interactively add, list, evaluate,
filter, and delete house records in a loop-driven menu.

FEATURES
--------
• Add new properties with:
    – Total square meters
    – Number of bedrooms
    – Number of bathrooms
    – Garden size (m²)
    – House type (Apartment / Detached / Semi-Detached / Townhouse)

• List all properties with detailed information  
• Evaluate each property:
    – Spacious vs. Cozy
    – Has garden vs. No garden

• Filter by bedrooms (≥ 3)  
• Delete by index

INSTALLATION & USAGE
--------------------
Clone the repository:
    git clone https://github.com/mikel0x/rustify_home_eval.git
    cd rustify_home_eval

Build and run:
    cargo run

Follow the on-screen menu prompts to manage your properties.

PROJECT STRUCTURE
-----------------
rustify_home_eval/
├── src/
│   ├── main.rs       entry point & menu loop  
│   ├── property.rs   House struct, HouseType enum, input logic  
│   └── utils.rs      CLI input helpers  
└── Cargo.toml        project manifest  

SUPPORTED PLATFORMS
-------------------
• Linux  
• macOS  
• Windows (via PowerShell / CMD)  

CONTRIBUTING
------------
If you find bugs, have improvements or new feature ideas—additional filters,
persistence, tests, etc.—please open an issue or submit a pull request on GitHub.

LICENSE
-------
This project is released under the MIT License (see LICENSE file).
