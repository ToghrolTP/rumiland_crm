use serde::{Deserialize, Serialize};

/// Iranian cities with their Persian names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IranCity {
    Zanjan,
    Abhar,
    Khorramdarreh,
    Gheydar,
    Hidaj,
    Saeenghale,
    Abbar,
    Soltanie,
    Sajas,
    Mahneshan,
    Sahrood,
    Garmab,
    Dandi,
    Zarinrood,
    Nourbahar,
    Karasf,
    Zarinabad,
    Armaghane,
    Chavarzaq,
    Halab,
}

impl IranCity {
    /// Get Persian name of the city
    pub fn persian_name(&self) -> &'static str {
        match self {
            IranCity::Zanjan => "زنجان",
            IranCity::Abhar => "ابهر",
            IranCity::Khorramdarreh => "خرمدره",
            IranCity::Gheydar => "قیدار",
            IranCity::Hidaj => "هیدج",
            IranCity::Saeenghale => "صاعین قلعه",
            IranCity::Abbar => "آب بر",
            IranCity::Soltanie => "سلطانیه",
            IranCity::Sajas => "سجاس",
            IranCity::Mahneshan => "ماهنشان",
            IranCity::Sahrood => "سهرود",
            IranCity::Garmab => "گرماب",
            IranCity::Dandi => "دندی",
            IranCity::Zarinrood => "زرین رود",
            IranCity::Nourbahar => "نوربهار",
            IranCity::Karasf => "کرسف",
            IranCity::Zarinabad => "زرین آباد",
            IranCity::Armaghane => "ارمغانه",
            IranCity::Chavarzaq => "چورزق",
            IranCity::Halab => "حلب",
        }
    }

    /// Get English name of the city (for storage)
    pub fn english_name(&self) -> &'static str {
        match self {
            IranCity::Zanjan => "Zanjan",
            IranCity::Abhar => "Abhar",
            IranCity::Khorramdarreh => "Khorramdarreh",
            IranCity::Gheydar => "Gheydar",
            IranCity::Hidaj => "Hidaj",
            IranCity::Saeenghale => "Saeenghale",
            IranCity::Abbar => "Abbar",
            IranCity::Soltanie => "Soltanie",
            IranCity::Sajas => "Sajas",
            IranCity::Mahneshan => "Mahneshan",
            IranCity::Sahrood => "Sahrood",
            IranCity::Garmab => "Garmab",
            IranCity::Dandi => "Dandi",
            IranCity::Zarinrood => "Zarinrood",
            IranCity::Nourbahar => "Nourbahar",
            IranCity::Karasf => "Karasf",
            IranCity::Zarinabad => "Zarinabad",
            IranCity::Armaghane => "Armaghane",
            IranCity::Chavarzaq => "Chavarzaq",
            IranCity::Halab => "Halab",
        }
    }

    /// Get all cities sorted by Persian name
    pub fn all() -> Vec<IranCity> {
        use IranCity::*;
        let mut cities = vec![
            Zanjan,
                        Abhar,
                        Khorramdarreh,
                        Gheydar,
                        Hidaj,
                        Saeenghale,
                        Abbar,
                        Soltanie,
                        Sajas,
                        Mahneshan,
                        Sahrood,
                        Garmab,
                        Dandi,
                        Zarinrood,
                        Nourbahar,
                        Karasf,
                        Zarinabad,
                        Armaghane,
                        Chavarzaq,
                        Halab,
        ];

        // Sort by Persian name for better UX
        cities.sort_by(|a, b| a.persian_name().cmp(b.persian_name()));
        cities
    }

    /// Parse city from string (English name)
    pub fn from_str(s: &str) -> Option<IranCity> {
        Self::all()
            .into_iter()
            .find(|city| city.english_name() == s)
    }
}

/// Alternative: Store cities as simple structs if you need more flexibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub code: String,
    pub name_fa: String,
    pub name_en: String,
    pub province: String,
}

impl City {
    /// Get major Iranian cities
    pub fn iranian_cities() -> Vec<City> {
        vec![
            City {
                code: "TEH".to_string(),
                name_fa: "تهران".to_string(),
                name_en: "Tehran".to_string(),
                province: "تهران".to_string(),
            },
            City {
                code: "ISF".to_string(),
                name_fa: "اصفهان".to_string(),
                name_en: "Isfahan".to_string(),
                province: "اصفهان".to_string(),
            },
            City {
                code: "SHZ".to_string(),
                name_fa: "شیراز".to_string(),
                name_en: "Shiraz".to_string(),
                province: "فارس".to_string(),
            },
            City {
                code: "MHD".to_string(),
                name_fa: "مشهد".to_string(),
                name_en: "Mashhad".to_string(),
                province: "خراسان رضوی".to_string(),
            },
            City {
                code: "TBZ".to_string(),
                name_fa: "تبریز".to_string(),
                name_en: "Tabriz".to_string(),
                province: "آذربایجان شرقی".to_string(),
            },
            City {
                code: "KRJ".to_string(),
                name_fa: "کرج".to_string(),
                name_en: "Karaj".to_string(),
                province: "البرز".to_string(),
            },
            City {
                code: "QOM".to_string(),
                name_fa: "قم".to_string(),
                name_en: "Qom".to_string(),
                province: "قم".to_string(),
            },
            City {
                code: "AHZ".to_string(),
                name_fa: "اهواز".to_string(),
                name_en: "Ahvaz".to_string(),
                province: "خوزستان".to_string(),
            },
            // Add more cities as needed
        ]
    }
}
