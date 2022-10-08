mod m_enum {
    fn enums1() {
        const EUROPE: u8 = 0;
        const ASIA: u8 = 1;
        const AFRICA: u8 = 2;
        const AMERICA: u8 = 3;
        const OCEANIA: u8 = 4;
        let continent = ASIA;
        if continent == EUROPE { print!("E"); }
        else if continent == ASIA { print!("As"); }
        else if continent == AFRICA { print!("Af"); }
        else if continent == AMERICA { print!("Am"); }
        else if continent == OCEANIA { print!("O"); }


    }

    fn nums2() {
        const EUROPE: u8 = 0;
        const ASIA: u8 = 1;
        const AFRICA: u8 = 2;
        const AMERICA: u8 = 3;
        const OCEANIA: u8 = 4;

        let continent = ASIA;

        if continent == EUROPE { print!("E"); }
        else if continent == ASIA { print!("As"); }
        else if continent == AFRICA { print!("Af"); }
        else if continent == AMERICA { print!("Am"); }
        else if continent == OCEANIA { print!("O"); }

        enum Continent {
            EUROPE, AFRICA, ASIA
        }

        let contin = Continent::AFRICA;

        match contin {
            Continent::ASIA => print!("AS"),
            Continent::AFRICA => print!("AF"),
            _ => {}
        }
    }
}