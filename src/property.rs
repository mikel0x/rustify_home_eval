#[derive(Debug)]
pub enum HouseType {
    Apartment,
    Detached,
    SemiDetached,
    Townhouse,
}

impl HouseType {
    pub fn to_string(&self) -> &str {
        match self {
            HouseType::Apartment => "Apartment",
            HouseType::Detached => "Detached",
            HouseType::SemiDetached => "Semi-Detached",
            HouseType::Townhouse => "Townhouse",
        }
    }
}

#[derive(Debug)]
pub struct House {
    pub square_meters: u32,
    pub bedrooms: u8,
    pub bathrooms: u8,
    pub garden_m2: u32,
    pub house_type: HouseType,
}

impl House {
    pub fn is_spacious(&self) -> bool {
        self.square_meters > 100 && self.bedrooms > 2
    }

    pub fn has_garden(&self) -> bool {
        self.garden_m2 > 0
    }

    pub fn describe_type(&self) -> &str {
        self.house_type.to_string()
    }

    pub fn new_from_input() -> Self {
        use crate::utils::{input, parse_u32};

        println!("\nEnter house details:");

        let square_meters = parse_u32("Total square meters: ");
        let bedrooms = parse_u32("Number of bedrooms: ") as u8;
        let bathrooms = parse_u32("Number of bathrooms: ") as u8;
        let garden_m2 = parse_u32("Garden size (m²): ");

        let house_type = loop {
            let input_type = input(
                "House type (1=Apartment, 2=Detached, 3=Semi-Detached, 4=Townhouse): ",
            );
            match input_type.trim() {
                "1" => break HouseType::Apartment,
                "2" => break HouseType::Detached,
                "3" => break HouseType::SemiDetached,
                "4" => break HouseType::Townhouse,
                _ => println!("Invalid choice. Please enter 1, 2, 3, or 4."),
            }
        };

        House {
            square_meters,
            bedrooms,
            bathrooms,
            garden_m2,
            house_type,
        }
    }
}