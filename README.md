<p align="center">
  <pre>
   ____            _   _      _   _ _           _   _             
  |  _ \ ___  __ _| | | | ___| | | (_) ___   __| | | |_ ___  _ __ 
  | |_) / _ \/ _` | | | |/ _ \ | | | |/ _ \ / _` | | __/ _ \| '__|
  |  _ <  __/ (_| | |_| |  __/ |_| | | (_) | (_| | | || (_) | |   
  |_| \_\___|\__,_|\___/ \___|\__|_|_|\___/ \__,_|  \__\___/|_|   
  </pre>
</p>

<p align="center">
  <a href="https://github.com/mikel0x/rustify_home_eval/actions"><img src="https://img.shields.io/github/actions/workflow/status/mikel0x/rustify_home_eval/ci.yml?branch=main" alt="Build Status"></a>
  <a href="https://github.com/mikel0x/rustify_home_eval/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-green.svg" alt="License"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-1.66+-orange.svg" alt="Rust Version"></a>
</p>

---

> **Rustify Home Evaluator**  
> A CLI property manager I built while working through the first nine chapters of *The Rust Programming Language*.

---

## Features

- **Add** new properties with:
  - Total square meters
  - Number of bedrooms
  - Number of bathrooms
  - Garden size (m²)
  - House type (Apartment / Detached / Semi-Detached / Townhouse)

- **List** all properties
- **Evaluate** each property:
  - Spacious vs. Cozy
  - Has garden vs. No garden
- **Filter** by bedrooms (≥ 3)
- **Delete** by index

---

## Installation & Usage

```bash
git clone https://github.com/mikel0x/rustify_home_eval.git
cd rustify_home_eval
cargo run
