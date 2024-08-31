pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

pub struct CharacterCode {
    name: String,
    birthday: String,
    age: String,
    profile: String,
    creator: String,
    favorite_color: String,
    favorite_food: String,
    location: String,
    personality: String,
    occupation: String,

    numbers: Vec<i32>,
    colors: Vec<Color>,
    numbers2: Vec<i32>,
}

static DEFCHAR: &str = "placeholder|||Coming soon!|||||||263|40|40|4|17|1|1|0|1|0|1|1|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|127|0|0|0|22|22|0|0|1|1|1|2|0|0|0|0|0|1|6|0|0|0|0|0|0|0|1|1|0|1|1|2|0|0|22|11|1|1|1|23|25|1|1|1|1|1|1|21|21|0|6|0|0|1|0|1|1|1|1|0|0|0|0|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|0|1|1|0|0|0|1|1|0|1|1|1|1|1|1|0|0|1|1|1|1|0|1|1|0|0|1|1|0|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|5|497|0|0|1|1|0|1|0|0|0|1|1|0|0|1|1|0|0|0|0|0|0|0|0|0|0|1|1|1|0|0|0|0|0|0|0|0|0|0|FFFFFF|6C71A4|8A6E5E|3A1F17|694F43|8A6E5E|3A1F17|694F43|8A6E5E|3A1F17|694F43|8A6E5E|3A1F17|694F43|8A6E5E|3A1F17|694F43|B15482|FFC2C2|855944|020202|27170F|855944|020202|27170F|A17261|3A1F17|A17261|3A1F17|8A624F|020202|8A624F|020202|191919|020202|ECECEC|4638FF|020202|BBD4FF|8589FF|020202|FF93BC|7F7EA6|020202|8AAEFF|FF8383|8589FF|FFC2C2|020202|FF8383|020202|FFFFFF|EDBDFF|4F03AA|FFFFFF|DEECFF|020202|3A82FF|EBE0FF|020202|8AAEFF|0256C9|020202|8AAEFF|E0E1FF|020202|8ACEFF|EDBDFF|4F03AA|E0FFFE|191919|020202|4638FF|AAA7CB|020202|EEE9FF|AAA7CB|020202|EEE9FF|EDBDFF|4F03AA|E0FFFE|EDBDFF|4F03AA|E0FFFE|FFFFFF|B2AEDB|FFFFFF|FFFFFF|B2AEDB|FFFFFF|E0F4FF|6605D9|EDBDFF|EDBDFF|4F03AA|E0FFFE|FFFFFF|020202|AAA7CB|FFFFFF|020202|AAA7CB|4F03AA|4F03AA|EDBDFF|4F03AA|4F03AA|EDBDFF|E0FFFE|6605D9|FFFFFF|E0FFFE|6605D9|FFFFFF|4638FF|020202|BCBBFF|B2AEDB|B2AEDB|DEECFF|E0FFFE|6605D9|E0FFFE|FFFFFF|020202|A487FF|FFFFFF|020202|A487FF|FF3F3F|020202|FFC2C2|FF3F3F|020202|FFFFFF|FF3F3F|020202|FFFFFF|FF3F3F|020202|191919|8589FF|020202|FFFFFF|8589FF|020202|FFFFFF|FFFFFF|B2AEDB|E0E1FF|FFFFFF|B2AEDB|E0E1FF|020202|020202|020202|FFFFFF|020202|FFFFFF|020202|020202|020202|020202|020202|020202|FFFFFF|FFFFFF|100|-100|1|1|360|100|-100|1|1|360|100|-100|1|1|360|100|-100|1|1|360|100|-100|1|1|360|100|-100|1|1|360|1";
static MAXLENGTH: usize = 478 + 29;

impl CharacterCode {
    pub fn new_from_code(mycode: &str) -> Result<Self, String> {
        let mut params: Vec<String> = mycode
            .trim()
            .split('|')
            .map(|s| s.trim().to_string())
            .collect();
        let def_params: Vec<String> = DEFCHAR
            .trim()
            .split('|')
            .map(|s| s.trim().to_string())
            .collect();

        if params.len() < 445 || params.len() > MAXLENGTH {
            return Err("Wrong size".to_owned());
        }
        for param in params.iter_mut() {
            if param.is_empty() {
                param.push('-');
            }
        }

        while params.len() < 478 {
            params.push(def_params[params.len()].to_owned());
        }
        while params.len() < MAXLENGTH {
            params.push(0.to_string());
        }

        let mut numbers = Vec::new();
        let mut colors = Vec::new();
        let mut numbers2 = Vec::new();

        for param in params[10..=278].iter() {
            match param.parse::<i32>() {
                Ok(num) => numbers.push(num),
                Err(err) => return Err(err.to_string()),
            };
        }
        for param in params[279..=446].iter() {
            match Color::new_from_hex(&param) {
                Ok(color) => colors.push(color),
                Err(err) => return Err(err),
            };
        }
        for param in params[447..MAXLENGTH].iter() {
            match param.parse::<i32>() {
                Ok(num) => numbers2.push(num),
                Err(err) => return Err(err.to_string()),
            };
        }

        let character = Self {
            name: params[0].to_owned(),
            birthday: params[1].to_owned(),
            age: params[2].to_owned(),
            profile: params[3].to_owned(),
            creator: params[4].to_owned(),
            favorite_color: params[5].to_owned(),
            favorite_food: params[6].to_owned(),
            location: params[7].to_owned(),
            personality: params[8].to_owned(),
            occupation: params[9].to_owned(),

            numbers,
            colors,
            numbers2,
        };

        if character.name.len() > 24
            || character.birthday.len() > 12
            || character.age.len() > 5
            || character.profile.len() > 300
            || character.creator.len() > 24
            || character.favorite_color.len() > 24
            || character.favorite_food.len() > 24
            || character.location.len() > 24
            || character.personality.len() > 24
            || character.occupation.len() > 24
        {
            Err("Wrong param length".to_owned())
        } else {
            Ok(character)
        }
    }
    pub fn to_code(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.name,
            self.birthday,
            self.age,
            self.profile,
            self.creator,
            self.favorite_color,
            self.favorite_food,
            self.location,
            self.personality,
            self.occupation,
            self.numbers
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join("|"),
            self.colors
                .iter()
                .map(|color| color.to_hex())
                .collect::<Vec<String>>()
                .join("|"),
            self.numbers2
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join("|"),
        )
    }
}

impl Color {
    pub fn new_from_hex(hex: &str) -> Result<Self, String> {
        let mut hex = hex.trim();
        if hex.starts_with("0x") {
            hex = &hex[2..];
        }

        let hex = if hex.ends_with("defined") {
            "FFFFFF"
        } else {
            hex
        };

        if hex.len() != 6 {
            return Err("Invalid hex length".to_string());
        }

        let red = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid value for red")?;
        let green = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid value for green")?;
        let blue = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid value for blue")?;

        Ok(Self { red, green, blue })
    }
    pub fn to_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.red, self.green, self.blue).to_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_test() {
        let hex = "ABCDEF";
        let color = Color::new_from_hex(hex);
        assert!(color.is_ok());
        assert_eq!(hex, color.unwrap().to_hex());
    }
    #[test]
    fn undefined_color_test() {
        let hex = "undefined";
        let color = Color::new_from_hex(hex);
        assert!(color.is_ok());
        assert_eq!("FFFFFF", color.unwrap().to_hex());
    }
    #[test]
    fn defined_color_test() {
        let hex = "defined";
        let color = Color::new_from_hex(hex);
        assert!(color.is_ok());
        assert_eq!("FFFFFF", color.unwrap().to_hex());
    }
    #[test]
    #[should_panic]
    fn defined_wrong_color_test() {
        let hex = "definedooooo";
        let color = Color::new_from_hex(hex);
        assert!(color.is_ok());
        assert_eq!("FFFFFF", color.unwrap().to_hex());
    }
    #[test]
    fn color_test_from_0xcolor() {
        let hex = "0xABCDEF";
        let target_hex = "ABCDEF";
        let color = Color::new_from_hex(hex);
        assert!(color.is_ok());
        assert_eq!(target_hex, color.unwrap().to_hex());
    }
    #[test]
    #[should_panic]
    fn color_test_value_fail() {
        let hex = "ABCDET";
        let color = Color::new_from_hex(hex);
        assert!(color.is_ok());
        assert_eq!(hex, color.unwrap().to_hex());
    }
    #[test]
    #[should_panic]
    fn color_test_length_fail() {
        let hex = "ABCDE00";
        let color = Color::new_from_hex(hex);
        assert!(color.is_ok());
        assert_eq!(hex, color.unwrap().to_hex());
    }

    #[test]
    fn character_test() {
        let code = "Nameeeeeeeeeeeeeeeeeeeee|Bdayyyyyyyyy|ageee|profileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofil|Creatorrrrrrrrrrrrrrrrrr|Fav_colorrrrrrrrrrrrrrrr|Fav_fooooooooooooooooood|locationnnnnnnnnnnnnnnnn|personalityyyyyyyyyyyyyy|Occupationnnnnnnnnnnnnnn|263|40|72|5|2|2|38|0|1|1|1|1|17|5|0|1|0|9|9|1|1|1|1|53|0|0|0|0|4|0|0|0|0|9|12|2|2|25|25|17|17|12|12|0|0|0|0|0|0|0|0|0|0|0|0|0|0|184|0|1|1|0|1|1|1|0|1|1|1|1|1|1|1|2|1|1|1|1|1|1|0|0|0|6|0|0|1|0|1|1|1|1|0|0|0|0|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|1|1|1|1|0|0|1|1|1|1|0|1|1|0|0|1|1|0|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|0|0|1|1|0|1|0|0|0|1|1|0|0|1|1|0|0|0|0|0|0|0|0|0|0|1|1|1|0|0|2|0|0|0|0|0|0|0|FFE2D4|8A624F|E0F4FF|414D69|8EFBFF|E0F4FF|414D69|8EFBFF|8A6E5E|414D69|694F43|E0F4FF|414D69|8EFBFF|8A6E5E|414D69|694F43|4C97D9|FFFFFF|8ACEFF|020202|020202|8ACEFF|020202|020202|3A82FF|020202|3A82FF|020202|8A624F|020202|8A624F|020202|191919|020202|ECECEC|4638FF|020202|BBD4FF|8589FF|020202|FF93BC|7F7EA6|020202|8AAEFF|FF8383|8589FF|FFC2C2|020202|FF8383|020202|FFFFFF|252C3C|020202|4C97D9|DEECFF|020202|3A82FF|EBE0FF|020202|8AAEFF|0256C9|020202|8AAEFF|E0E1FF|020202|8ACEFF|8ACEFF|020202|FFFFFF|5478E0|020202|8ACEFF|AAA7CB|020202|EEE9FF|AAA7CB|020202|EEE9FF|5478E0|020202|8ACEFF|5478E0|020202|8ACEFF|FFFFFF|020202|FFFFFF|FFFFFF|020202|FFFFFF|191919|020202|B4BFCD|191919|020202|B8B8B8|FFFFFF|020202|AAA7CB|FFFFFF|020202|AAA7CB|191919|020202|5478E0|191919|020202|5478E0|FFFFFF|020202|8ACEFF|FFFFFF|020202|8ACEFF|4638FF|020202|BCBBFF|8589FF|020202|FFFFFF|A487FF|020202|8AAEFF|FFFFFF|020202|A487FF|FFFFFF|020202|A487FF|FF3F3F|020202|FFC2C2|F7D095|020202|664430|FF3F3F|020202|FFFFFF|FF3F3F|020202|191919|8589FF|020202|FFFFFF|8589FF|020202|FFFFFF|AAA7CB|020202|B4BFCD|AAA7CB|020202|B4BFCD|020202|020202|020202|FFFFFF|020202|FFFFFF|020202|020202|020202|020202|020202|020202";
        let character = CharacterCode::new_from_code(code);
        assert!(character.is_ok());
        assert!(character.unwrap().to_code().contains(code));
    }

    #[test]
    #[should_panic]
    fn character_test_length_fail() {
        let code = "Nameeeeeeeeeeeeeeeeeeeee|Bdayyyyyyyyy|ageee00|profileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofil|Creatorrrrrrrrrrrrrrrrrr|Fav_colorrrrrrrrrrrrrrrr|Fav_fooooooooooooooooood|locationnnnnnnnnnnnnnnnn|personalityyyyyyyyyyyyyy|Occupationnnnnnnnnnnnnnn|263|40|72|5|2|2|38|0|1|1|1|1|17|5|0|1|0|9|9|1|1|1|1|53|0|0|0|0|4|0|0|0|0|9|12|2|2|25|25|17|17|12|12|0|0|0|0|0|0|0|0|0|0|0|0|0|0|184|0|1|1|0|1|1|1|0|1|1|1|1|1|1|1|2|1|1|1|1|1|1|0|0|0|6|0|0|1|0|1|1|1|1|0|0|0|0|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|1|1|1|1|0|0|1|1|1|1|0|1|1|0|0|1|1|0|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|0|0|1|1|0|1|0|0|0|1|1|0|0|1|1|0|0|0|0|0|0|0|0|0|0|1|1|1|0|0|2|0|0|0|0|0|0|0|FFE2D4|8A624F|E0F4FF|414D69|8EFBFF|E0F4FF|414D69|8EFBFF|8A6E5E|414D69|694F43|E0F4FF|414D69|8EFBFF|8A6E5E|414D69|694F43|4C97D9|FFFFFF|8ACEFF|020202|020202|8ACEFF|020202|020202|3A82FF|020202|3A82FF|020202|8A624F|020202|8A624F|020202|191919|020202|ECECEC|4638FF|020202|BBD4FF|8589FF|020202|FF93BC|7F7EA6|020202|8AAEFF|FF8383|8589FF|FFC2C2|020202|FF8383|020202|FFFFFF|252C3C|020202|4C97D9|DEECFF|020202|3A82FF|EBE0FF|020202|8AAEFF|0256C9|020202|8AAEFF|E0E1FF|020202|8ACEFF|8ACEFF|020202|FFFFFF|5478E0|020202|8ACEFF|AAA7CB|020202|EEE9FF|AAA7CB|020202|EEE9FF|5478E0|020202|8ACEFF|5478E0|020202|8ACEFF|FFFFFF|020202|FFFFFF|FFFFFF|020202|FFFFFF|191919|020202|B4BFCD|191919|020202|B8B8B8|FFFFFF|020202|AAA7CB|FFFFFF|020202|AAA7CB|191919|020202|5478E0|191919|020202|5478E0|FFFFFF|020202|8ACEFF|FFFFFF|020202|8ACEFF|4638FF|020202|BCBBFF|8589FF|020202|FFFFFF|A487FF|020202|8AAEFF|FFFFFF|020202|A487FF|FFFFFF|020202|A487FF|FF3F3F|020202|FFC2C2|F7D095|020202|664430|FF3F3F|020202|FFFFFF|FF3F3F|020202|191919|8589FF|020202|FFFFFF|8589FF|020202|FFFFFF|AAA7CB|020202|B4BFCD|AAA7CB|020202|B4BFCD|020202|020202|020202|FFFFFF|020202|FFFFFF|020202|020202|020202|020202|020202|020202";
        let character = CharacterCode::new_from_code(code);
        assert!(character.is_ok());
        assert!(character.unwrap().to_code().contains(code));
    }
    #[test]
    #[should_panic]
    fn character_test_number_value_fail() {
        let code = "Nameeeeeeeeeeeeeeeeeeeee|Bdayyyyyyyyy|ageee|profileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofil|Creatorrrrrrrrrrrrrrrrrr|Fav_colorrrrrrrrrrrrrrrr|Fav_fooooooooooooooooood|locationnnnnnnnnnnnnnnnn|personalityyyyyyyyyyyyyy|Occupationnnnnnnnnnnnnnn|263p|40|72|5|2|2|38|0|1|1|1|1|17|5|0|1|0|9|9|1|1|1|1|53|0|0|0|0|4|0|0|0|0|9|12|2|2|25|25|17|17|12|12|0|0|0|0|0|0|0|0|0|0|0|0|0|0|184|0|1|1|0|1|1|1|0|1|1|1|1|1|1|1|2|1|1|1|1|1|1|0|0|0|6|0|0|1|0|1|1|1|1|0|0|0|0|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|1|1|1|1|0|0|1|1|1|1|0|1|1|0|0|1|1|0|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|0|0|1|1|0|1|0|0|0|1|1|0|0|1|1|0|0|0|0|0|0|0|0|0|0|1|1|1|0|0|2|0|0|0|0|0|0|0|FFE2D4|8A624F|E0F4FF|414D69|8EFBFF|E0F4FF|414D69|8EFBFF|8A6E5E|414D69|694F43|E0F4FF|414D69|8EFBFF|8A6E5E|414D69|694F43|4C97D9|FFFFFF|8ACEFF|020202|020202|8ACEFF|020202|020202|3A82FF|020202|3A82FF|020202|8A624F|020202|8A624F|020202|191919|020202|ECECEC|4638FF|020202|BBD4FF|8589FF|020202|FF93BC|7F7EA6|020202|8AAEFF|FF8383|8589FF|FFC2C2|020202|FF8383|020202|FFFFFF|252C3C|020202|4C97D9|DEECFF|020202|3A82FF|EBE0FF|020202|8AAEFF|0256C9|020202|8AAEFF|E0E1FF|020202|8ACEFF|8ACEFF|020202|FFFFFF|5478E0|020202|8ACEFF|AAA7CB|020202|EEE9FF|AAA7CB|020202|EEE9FF|5478E0|020202|8ACEFF|5478E0|020202|8ACEFF|FFFFFF|020202|FFFFFF|FFFFFF|020202|FFFFFF|191919|020202|B4BFCD|191919|020202|B8B8B8|FFFFFF|020202|AAA7CB|FFFFFF|020202|AAA7CB|191919|020202|5478E0|191919|020202|5478E0|FFFFFF|020202|8ACEFF|FFFFFF|020202|8ACEFF|4638FF|020202|BCBBFF|8589FF|020202|FFFFFF|A487FF|020202|8AAEFF|FFFFFF|020202|A487FF|FFFFFF|020202|A487FF|FF3F3F|020202|FFC2C2|F7D095|020202|664430|FF3F3F|020202|FFFFFF|FF3F3F|020202|191919|8589FF|020202|FFFFFF|8589FF|020202|FFFFFF|AAA7CB|020202|B4BFCD|AAA7CB|020202|B4BFCD|020202|020202|020202|FFFFFF|020202|FFFFFF|020202|020202|020202|020202|020202|020202";
        let character = CharacterCode::new_from_code(code);
        assert!(character.is_ok());
        assert!(character.unwrap().to_code().contains(code));
    }
    #[test]
    #[should_panic]
    fn character_test_color_value_fail() {
        let code = "Nameeeeeeeeeeeeeeeeeeeee|Bdayyyyyyyyy|ageee|profileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofil|Creatorrrrrrrrrrrrrrrrrr|Fav_colorrrrrrrrrrrrrrrr|Fav_fooooooooooooooooood|locationnnnnnnnnnnnnnnnn|personalityyyyyyyyyyyyyy|Occupationnnnnnnnnnnnnnn|263|40|72|5|2|2|38|0|1|1|1|1|17|5|0|1|0|9|9|1|1|1|1|53|0|0|0|0|4|0|0|0|0|9|12|2|2|25|25|17|17|12|12|0|0|0|0|0|0|0|0|0|0|0|0|0|0|184|0|1|1|0|1|1|1|0|1|1|1|1|1|1|1|2|1|1|1|1|1|1|0|0|0|6|0|0|1|0|1|1|1|1|0|0|0|0|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|1|1|1|1|0|0|1|1|1|1|0|1|1|0|0|1|1|0|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|0|0|1|1|0|1|0|0|0|1|1|0|0|1|1|0|0|0|0|0|0|0|0|0|0|1|1|1|0|0|2|0|0|0|0|0|0|0|FFE2D4|8A624F|E0F4FF|414D69|8EFBFF|E0F4FF|414D69|8EFBFF|8A6E5E|414D69|694F43|E0F4FF|414D69|8EFBFF|8A6E5E|414D69|694F43|4C97D9|FFFFFF|8ACEFF|020202|020202|8ACEFF|020202|020202|3A82FF|020202|3A82FF|020202|8A624F|020202|8A624F|020202|191919|020202|ECECEC|4638FF|020202|BBD4FF|8589FF|020202|FF93BC|7F7EA6|020202|8AAEFF|FF8383|8589FF|FFC2C2|020202|FF8383|020202|FFFFFF|252C3C|020202|4C97D9|DEECFF|020202|3A82FF|EBE0FF|020202|8AAEFF|0256C9|020202|8AAEFF|E0E1FF|020202|8ACEFF|8ACEFF|020202|FFFFFF|5478E0|020202|8ACEFF|AAA7CB|020202|EEE9FF|AAA7CB|020202|EEE9FF|5478E0|020202|8ACEFF|5478E0|020202|8ACEFF|FFFFFF|020202|FFFFFF|FFFFFF|020202|FFFFFF|191919|020202|B4BFCD|191919|020202|B8B8B8|FFFFFF|020202|AAA7CB|FFFFFF|020202|AAA7CB|191919|020202|5478E0|191919|020202|5478E0|FFFFFF|020202|8ACEFF|FFFFFF|020202|8ACEFF|4638FF|020202|BCBBFF|8589FF|020202|FFFFFF|A487FF|020202|8AAEFF|FFFFFF|020202|A487FF|FFFFFF|020202|A487FF|FF3F3F|020202|FFC2C2|F7D095|020202|664430|FF3F3F|020202|FFFFFF|FF3F3F|020202|191919|8589FF|020202|FFFFFF|8589FF|020202|FFFFFF|AAA7CB|020202|B4BFCD|AAA7CB|020202|B4BFCD|020202|020202|020202|FFFFFF|020202|FFFFFF|0202T2|020202|020202|020202|020202|020202";
        let character = CharacterCode::new_from_code(code);
        assert!(character.is_ok());
        assert!(character.unwrap().to_code().contains(code));
    }
    #[test]
    fn character_test_empty() {
        let code = "Nameeeeeeeeeeeeeeeeeeeee|Bdayyyyyyyyy||profileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofil|Creatorrrrrrrrrrrrrrrrrr|Fav_colorrrrrrrrrrrrrrrr|Fav_fooooooooooooooooood|locationnnnnnnnnnnnnnnnn|personalityyyyyyyyyyyyyy|Occupationnnnnnnnnnnnnnn|263|40|72|5|2|2|38|0|1|1|1|1|17|5|0|1|0|9|9|1|1|1|1|53|0|0|0|0|4|0|0|0|0|9|12|2|2|25|25|17|17|12|12|0|0|0|0|0|0|0|0|0|0|0|0|0|0|184|0|1|1|0|1|1|1|0|1|1|1|1|1|1|1|2|1|1|1|1|1|1|0|0|0|6|0|0|1|0|1|1|1|1|0|0|0|0|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|1|1|1|1|0|0|1|1|1|1|0|1|1|0|0|1|1|0|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|0|0|1|1|0|1|0|0|0|1|1|0|0|1|1|0|0|0|0|0|0|0|0|0|0|1|1|1|0|0|2|0|0|0|0|0|0|0|FFE2D4|8A624F|E0F4FF|414D69|8EFBFF|E0F4FF|414D69|8EFBFF|8A6E5E|414D69|694F43|E0F4FF|414D69|8EFBFF|8A6E5E|414D69|694F43|4C97D9|FFFFFF|8ACEFF|020202|020202|8ACEFF|020202|020202|3A82FF|020202|3A82FF|020202|8A624F|020202|8A624F|020202|191919|020202|ECECEC|4638FF|020202|BBD4FF|8589FF|020202|FF93BC|7F7EA6|020202|8AAEFF|FF8383|8589FF|FFC2C2|020202|FF8383|020202|FFFFFF|252C3C|020202|4C97D9|DEECFF|020202|3A82FF|EBE0FF|020202|8AAEFF|0256C9|020202|8AAEFF|E0E1FF|020202|8ACEFF|8ACEFF|020202|FFFFFF|5478E0|020202|8ACEFF|AAA7CB|020202|EEE9FF|AAA7CB|020202|EEE9FF|5478E0|020202|8ACEFF|5478E0|020202|8ACEFF|FFFFFF|020202|FFFFFF|FFFFFF|020202|FFFFFF|191919|020202|B4BFCD|191919|020202|B8B8B8|FFFFFF|020202|AAA7CB|FFFFFF|020202|AAA7CB|191919|020202|5478E0|191919|020202|5478E0|FFFFFF|020202|8ACEFF|FFFFFF|020202|8ACEFF|4638FF|020202|BCBBFF|8589FF|020202|FFFFFF|A487FF|020202|8AAEFF|FFFFFF|020202|A487FF|FFFFFF|020202|A487FF|FF3F3F|020202|FFC2C2|F7D095|020202|664430|FF3F3F|020202|FFFFFF|FF3F3F|020202|191919|8589FF|020202|FFFFFF|8589FF|020202|FFFFFF|AAA7CB|020202|B4BFCD|AAA7CB|020202|B4BFCD|020202|020202|020202|FFFFFF|020202|FFFFFF|020202|020202|020202|020202|020202|020202";
        let target_code = "Nameeeeeeeeeeeeeeeeeeeee|Bdayyyyyyyyy|-|profileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofileprofil|Creatorrrrrrrrrrrrrrrrrr|Fav_colorrrrrrrrrrrrrrrr|Fav_fooooooooooooooooood|locationnnnnnnnnnnnnnnnn|personalityyyyyyyyyyyyyy|Occupationnnnnnnnnnnnnnn|263|40|72|5|2|2|38|0|1|1|1|1|17|5|0|1|0|9|9|1|1|1|1|53|0|0|0|0|4|0|0|0|0|9|12|2|2|25|25|17|17|12|12|0|0|0|0|0|0|0|0|0|0|0|0|0|0|184|0|1|1|0|1|1|1|0|1|1|1|1|1|1|1|2|1|1|1|1|1|1|0|0|0|6|0|0|1|0|1|1|1|1|0|0|0|0|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|1|1|1|1|0|0|1|1|1|1|0|1|1|0|0|1|1|0|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|0|0|1|1|0|1|0|0|0|1|1|0|0|1|1|0|0|0|0|0|0|0|0|0|0|1|1|1|0|0|2|0|0|0|0|0|0|0|FFE2D4|8A624F|E0F4FF|414D69|8EFBFF|E0F4FF|414D69|8EFBFF|8A6E5E|414D69|694F43|E0F4FF|414D69|8EFBFF|8A6E5E|414D69|694F43|4C97D9|FFFFFF|8ACEFF|020202|020202|8ACEFF|020202|020202|3A82FF|020202|3A82FF|020202|8A624F|020202|8A624F|020202|191919|020202|ECECEC|4638FF|020202|BBD4FF|8589FF|020202|FF93BC|7F7EA6|020202|8AAEFF|FF8383|8589FF|FFC2C2|020202|FF8383|020202|FFFFFF|252C3C|020202|4C97D9|DEECFF|020202|3A82FF|EBE0FF|020202|8AAEFF|0256C9|020202|8AAEFF|E0E1FF|020202|8ACEFF|8ACEFF|020202|FFFFFF|5478E0|020202|8ACEFF|AAA7CB|020202|EEE9FF|AAA7CB|020202|EEE9FF|5478E0|020202|8ACEFF|5478E0|020202|8ACEFF|FFFFFF|020202|FFFFFF|FFFFFF|020202|FFFFFF|191919|020202|B4BFCD|191919|020202|B8B8B8|FFFFFF|020202|AAA7CB|FFFFFF|020202|AAA7CB|191919|020202|5478E0|191919|020202|5478E0|FFFFFF|020202|8ACEFF|FFFFFF|020202|8ACEFF|4638FF|020202|BCBBFF|8589FF|020202|FFFFFF|A487FF|020202|8AAEFF|FFFFFF|020202|A487FF|FFFFFF|020202|A487FF|FF3F3F|020202|FFC2C2|F7D095|020202|664430|FF3F3F|020202|FFFFFF|FF3F3F|020202|191919|8589FF|020202|FFFFFF|8589FF|020202|FFFFFF|AAA7CB|020202|B4BFCD|AAA7CB|020202|B4BFCD|020202|020202|020202|FFFFFF|020202|FFFFFF|020202|020202|020202|020202|020202|020202";
        let character = CharacterCode::new_from_code(code);
        assert!(character.is_ok());
        assert!(character.unwrap().to_code().contains(target_code));
    }
    #[test]
    fn extra_2_color() {
        let code = "Default Boy|2/22|20|Hey! I'm the default boy of Gacha Club.|Lunime|Blue|Rice|USA|Funny|Student|2|1|1|4|1|1|1|0|1|1|1|1|0|1|0|0|0|1|1|1|1|1|1|1|0|0|0|0|0|0|0|0|0|2|0|1|1|1|1|3|3|1|1|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|1|1|0|1|1|1|0|0|1|1|1|1|1|1|2|1|1|1|1|1|1|0|0|1|6|26|0|1|0|1|1|1|1|0|0|0|0|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|1|1|1|1|0|0|1|1|1|1|0|1|1|0|0|1|1|0|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|0|0|1|1|0|1|0|0|0|1|1|0|0|1|1|0|0|0|0|0|0|0|0|0|0|1|1|1|0|0|0|0|0|0|0|0|0|0|FFE2D4|8A624F|8A6E5E|3A1F17|694F43|8A6E5E|3A1F17|694F43|8A6E5E|3A1F17|694F43|8A6E5E|3A1F17|694F43|8A6E5E|3A1F17|694F43|B15482|FFC2C2|855944|020202|27170F|855944|020202|27170F|A17261|3A1F17|A17261|3A1F17|8A624F|020202|8A624F|020202|191919|020202|ECECEC|4638FF|020202|BBD4FF|8589FF|020202|FF93BC|7F7EA6|020202|8AAEFF|FF8383|8589FF|FFC2C2|020202|FF8383|020202|FFFFFF|FFFFFF|020202|8AAEFF|DEECFF|020202|3A82FF|EBE0FF|020202|8AAEFF|0256C9|020202|8AAEFF|E0E1FF|020202|8ACEFF|FFFFFF|020202|8589FF|191919|020202|4638FF|AAA7CB|020202|EEE9FF|AAA7CB|020202|EEE9FF|FFFFFF|020202|8589FF|FFFFFF|020202|8589FF|3D3E62|020202|3D3E62|3D3E62|020202|3D3E62|191919|020202|B4BFCD|191919|020202|B8B8B8|FFFFFF|020202|AAA7CB|FFFFFF|020202|AAA7CB|8AAEFF|020202|DEECFF|8AAEFF|020202|DEECFF|FFFFFF|020202|3A82FF|FFFFFF|020202|3A82FF|4638FF|020202|BCBBFF|8589FF|020202|FFFFFF|A487FF|020202|8AAEFF|FFFFFF|020202|A487FF|FFFFFF|020202|A487FF|FF3F3F|020202|FFC2C2|FF3F3F|020202|FFFFFF|FF3F3F|020202|FFFFFF|FF3F3F|020202|191919|8589FF|020202|FFFFFF|8589FF|020202|FFFFFF|AAA7CB|020202|B4BFCD|AAA7CB|020202|B4BFCD|BF0000|020202|020202|FFFFFF|020202|FFFFFF|020202|020202|020202|020202|020202|020202|FFFFFF|FFFFFF";
        let character = CharacterCode::new_from_code(code);
        assert!(character.is_ok());
        assert!(character.unwrap().to_code().contains(code));
    }
    #[test]
    fn extra_10_num() {
        let code = "Default Boy|2/22|20|Hey! I'm the default boy of Gacha Club.|Lunime|Blue|Rice|USA|Funny|Student|2|1|1|4|1|1|1|0|1|1|1|1|0|1|0|0|0|1|1|1|1|1|1|1|0|0|0|0|0|0|0|0|0|2|0|1|1|1|1|3|3|1|1|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|1|1|0|1|1|1|0|0|1|1|1|1|1|1|2|1|1|1|1|1|1|0|0|1|6|26|0|1|0|1|1|1|1|0|0|0|0|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|1|1|1|1|0|0|1|1|1|1|0|1|1|0|0|1|1|0|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|0|0|1|1|0|1|0|0|0|1|1|0|0|1|1|0|0|0|0|0|0|0|0|0|0|1|1|1|0|0|0|0|0|0|0|0|0|0|FFE2D4|8A624F|8A6E5E|3A1F17|694F43|8A6E5E|3A1F17|694F43|8A6E5E|3A1F17|694F43|8A6E5E|3A1F17|694F43|8A6E5E|3A1F17|694F43|B15482|FFC2C2|855944|020202|27170F|855944|020202|27170F|A17261|3A1F17|A17261|3A1F17|8A624F|020202|8A624F|020202|191919|020202|ECECEC|4638FF|020202|BBD4FF|8589FF|020202|FF93BC|7F7EA6|020202|8AAEFF|FF8383|8589FF|FFC2C2|020202|FF8383|020202|FFFFFF|FFFFFF|020202|8AAEFF|DEECFF|020202|3A82FF|EBE0FF|020202|8AAEFF|0256C9|020202|8AAEFF|E0E1FF|020202|8ACEFF|FFFFFF|020202|8589FF|191919|020202|4638FF|AAA7CB|020202|EEE9FF|AAA7CB|020202|EEE9FF|FFFFFF|020202|8589FF|FFFFFF|020202|8589FF|3D3E62|020202|3D3E62|3D3E62|020202|3D3E62|191919|020202|B4BFCD|191919|020202|B8B8B8|FFFFFF|020202|AAA7CB|FFFFFF|020202|AAA7CB|8AAEFF|020202|DEECFF|8AAEFF|020202|DEECFF|FFFFFF|020202|3A82FF|FFFFFF|020202|3A82FF|4638FF|020202|BCBBFF|8589FF|020202|FFFFFF|A487FF|020202|8AAEFF|FFFFFF|020202|A487FF|FFFFFF|020202|A487FF|FF3F3F|020202|FFC2C2|FF3F3F|020202|FFFFFF|FF3F3F|020202|FFFFFF|FF3F3F|020202|191919|8589FF|020202|FFFFFF|8589FF|020202|FFFFFF|AAA7CB|020202|B4BFCD|AAA7CB|020202|B4BFCD|BF0000|020202|020202|FFFFFF|020202|FFFFFF|020202|020202|020202|020202|020202|020202|FFFFFF|FFFFFF|0|0|0|0|1|7|4|-1|10|4";
        let character = CharacterCode::new_from_code(code);
        assert!(character.is_ok());
        assert!(character.unwrap().to_code().contains(code));
    }
    #[test]
    fn new_character_test() {
        let code = "placeholder|-|-|Coming soon!|-|-|-|-|-|-|263|40|40|4|17|1|1|0|1|0|1|1|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|127|0|0|0|22|22|0|0|1|1|1|2|0|0|0|0|0|1|6|0|0|0|0|0|0|0|1|1|0|1|1|2|0|0|22|11|1|1|1|23|25|1|1|1|1|1|1|21|21|0|6|0|0|1|0|1|1|1|1|0|0|0|0|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|0|0|1|1|0|1|1|0|1|1|0|0|0|1|1|0|1|1|1|1|1|1|0|0|1|1|1|1|0|1|1|0|0|1|1|0|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|1|5|497|0|0|1|1|0|1|0|0|0|1|1|0|0|1|1|0|0|0|0|0|0|0|0|0|0|1|1|1|0|0|0|0|0|0|0|0|0|0|FFFFFF|6C71A4|8A6E5E|3A1F17|694F43|8A6E5E|3A1F17|694F43|8A6E5E|3A1F17|694F43|8A6E5E|3A1F17|694F43|8A6E5E|3A1F17|694F43|B15482|FFC2C2|855944|020202|27170F|855944|020202|27170F|A17261|3A1F17|A17261|3A1F17|8A624F|020202|8A624F|020202|191919|020202|ECECEC|4638FF|020202|BBD4FF|8589FF|020202|FF93BC|7F7EA6|020202|8AAEFF|FF8383|8589FF|FFC2C2|020202|FF8383|020202|FFFFFF|EDBDFF|4F03AA|FFFFFF|DEECFF|020202|3A82FF|EBE0FF|020202|8AAEFF|0256C9|020202|8AAEFF|E0E1FF|020202|8ACEFF|EDBDFF|4F03AA|E0FFFE|191919|020202|4638FF|AAA7CB|020202|EEE9FF|AAA7CB|020202|EEE9FF|EDBDFF|4F03AA|E0FFFE|EDBDFF|4F03AA|E0FFFE|FFFFFF|B2AEDB|FFFFFF|FFFFFF|B2AEDB|FFFFFF|E0F4FF|6605D9|EDBDFF|EDBDFF|4F03AA|E0FFFE|FFFFFF|020202|AAA7CB|FFFFFF|020202|AAA7CB|4F03AA|4F03AA|EDBDFF|4F03AA|4F03AA|EDBDFF|E0FFFE|6605D9|FFFFFF|E0FFFE|6605D9|FFFFFF|4638FF|020202|BCBBFF|B2AEDB|B2AEDB|DEECFF|E0FFFE|6605D9|E0FFFE|FFFFFF|020202|A487FF|FFFFFF|020202|A487FF|FF3F3F|020202|FFC2C2|FF3F3F|020202|FFFFFF|FF3F3F|020202|FFFFFF|FF3F3F|020202|191919|8589FF|020202|FFFFFF|8589FF|020202|FFFFFF|FFFFFF|B2AEDB|E0E1FF|FFFFFF|B2AEDB|E0E1FF|020202|020202|020202|FFFFFF|020202|FFFFFF|020202|020202|020202|020202|020202|020202|FFFFFF|FFFFFF|100|-100|1|1|360|100|-100|1|1|360|100|-100|1|1|360|100|-100|1|1|360|100|-100|1|1|360|100|-100|1|1|360|4";
        let character = CharacterCode::new_from_code(code);
        assert!(character.is_ok());
        assert_eq!(character.unwrap().to_code(), code);
    }
}
