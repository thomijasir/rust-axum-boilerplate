// Trait for converting between a struct and a delimited string
pub trait StructifyConversion: Sized {
    fn to_field_values(&self) -> Vec<String>;
    fn from_field_values(values: Vec<String>) -> Result<Self, String>;
}

// Function to convert a struct to a pipe-delimited string
pub fn to_string<T: StructifyConversion>(value: &T) -> String {
    value.to_field_values().join("|")
}

// Function to parse a pipe-delimited string back into a struct
pub fn from_string<T: StructifyConversion>(s: &str) -> Result<T, String> {
    let values: Vec<String> = s.split('|').map(String::from).collect();
    T::from_field_values(values)
}

// Helper trait to convert strings to various types
pub trait FromString: Sized {
    fn from_string(s: &str) -> Result<Self, String>;
}

// Implement FromString for common types
impl FromString for String {
    fn from_string(s: &str) -> Result<Self, String> {
        Ok(s.to_string())
    }
}

impl FromString for i32 {
    fn from_string(s: &str) -> Result<Self, String> {
        s.parse::<i32>().map_err(|e| e.to_string())
    }
}

impl FromString for u32 {
    fn from_string(s: &str) -> Result<Self, String> {
        s.parse::<u32>().map_err(|e| e.to_string())
    }
}

impl FromString for i64 {
    fn from_string(s: &str) -> Result<Self, String> {
        s.parse::<i64>().map_err(|e| e.to_string())
    }
}

impl FromString for u64 {
    fn from_string(s: &str) -> Result<Self, String> {
        s.parse::<u64>().map_err(|e| e.to_string())
    }
}

impl FromString for f32 {
    fn from_string(s: &str) -> Result<Self, String> {
        s.parse::<f32>().map_err(|e| e.to_string())
    }
}

impl FromString for f64 {
    fn from_string(s: &str) -> Result<Self, String> {
        s.parse::<f64>().map_err(|e| e.to_string())
    }
}

impl FromString for bool {
    fn from_string(s: &str) -> Result<Self, String> {
        s.parse::<bool>().map_err(|e| e.to_string())
    }
}

// Helper function to use the FromString trait
#[allow(dead_code)]
pub(crate) fn parse_from_string<T: FromString>(s: &str) -> Result<T, String> {
    T::from_string(s)
}

// The main macro for automatic implementation
#[macro_export]
macro_rules! structify {
    ($struct_name:ident, $($field:ident),+) => {
        impl $crate::utils::structify::StructifyConversion for $struct_name {
            fn to_field_values(&self) -> Vec<String> {
                vec![
                    $(self.$field.to_string()),+
                ]
            }

            fn from_field_values(values: Vec<String>) -> Result<Self, String> {
                let expected_len = [$(stringify!($field)),+].len();
                if values.len() != expected_len {
                    return Err(format!("Expected {} values, got {}", expected_len, values.len()));
                }

                let mut iter = values.iter();

                Ok($struct_name {
                    $($field: $crate::utils::structify::parse_from_string(iter.next().unwrap())
                        .map_err(|e| format!("Error parsing {}: {}", stringify!($field), e))?),+
                })
            }
        }
    };
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    pub struct UserCredentialsInfo {
        pub id: String,
        pub email: String,
    }

    crate::structify!(UserCredentialsInfo, id, email);

    pub struct Product {
        pub id: String,
        pub name: String,
        pub price: f64,
        pub count: u32,
        pub available: bool,
    }

    crate::structify!(Product, id, name, price, count, available);

    #[test]
    fn test_user_credentials() {
        let user = UserCredentialsInfo {
            id: "something".to_string(),
            email: "thomi@dot.com".to_string(),
        };

        let user_str = to_string(&user);
        assert_eq!(user_str, "something|thomi@dot.com");

        let parsed_user = from_string::<UserCredentialsInfo>(&user_str).unwrap();
        assert_eq!(parsed_user.id, "something");
        assert_eq!(parsed_user.email, "thomi@dot.com");
    }

    #[test]
    fn test_product() {
        let product = Product {
            id: "P12345".to_string(),
            name: "Laptop".to_string(),
            price: 999.99,
            count: 42,
            available: true,
        };

        let product_str = to_string(&product);
        assert_eq!(product_str, "P12345|Laptop|999.99|42|true");

        let parsed_product = from_string::<Product>(&product_str).unwrap();
        assert_eq!(parsed_product.id, "P12345");
        assert_eq!(parsed_product.name, "Laptop");
        assert_eq!(parsed_product.price, 999.99);
        assert_eq!(parsed_product.count, 42);
        assert!(parsed_product.available);
    }
}
