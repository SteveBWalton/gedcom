// Modulus to define the 'GedComDate' class.
// Rust does not have classes!

// Member variables for the 'GedComDate' class.
pub struct GedComDate {
    pub is_empty: bool,
    pub is_day_guess: bool,
    pub is_month_guess: bool,
    pub is_year_guess: bool,
    pub is_before: bool,
    pub is_after: bool,
    pub is_month_quarter: bool,
    pub the_date: DateTime,
}

