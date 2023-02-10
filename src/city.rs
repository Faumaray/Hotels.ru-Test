/// Представление базовой информации о городе
/// `name`: Название города
/// `country`: Страна в которой находится город
/// `abbreviation`: Аббревиатура города
#[derive(Clone, Copy, Default)]
pub struct City<'a> {
    name: &'a str,
    country: &'a str,
    abbreviation: &'a str,
}

#[allow(unused)]
impl<'a> City<'a> {
    pub fn new(name: &'a str, country: &'a str, abbreviation: &'a str) -> Self {
        Self {
            name,
            country,
            abbreviation,
        }
    }
    pub fn new_from_name(name: &'a str) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
    pub fn name(&self) -> &str {
        self.name
    }
    pub fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }
    pub fn country(&self) -> &str {
        self.country
    }

    pub fn abbreviation(&self) -> &str {
        self.abbreviation
    }
}

/// Создает строку из (городов)[City] где города разделены разделителем, а в конце стоит точка
/// `cities`: Массив городов
/// `delimiter`: Разделитель
#[allow(dead_code)]
pub fn format_city_names<'a>(
    cities: impl AsRef<[City<'a>]> + 'a,
    delimiter: &'a str,
) -> impl AsRef<str> {
    cities
        .as_ref()
        .iter()
        .map(|city| city.name())
        .collect::<Vec<_>>()
        .join(delimiter)
        + "."
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_format_city_name() {
        let cities = vec![
            City::new_from_name("Москва"),
            City::new_from_name("Санкт-Петербург"),
            City::new_from_name("Воронеж"),
        ];

        assert_eq!(
            "Москва, Санкт-Петербург, Воронеж.",
            format_city_names(cities, ", ").as_ref()
        );
    }
}
