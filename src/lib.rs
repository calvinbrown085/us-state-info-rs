use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
/// Enumeration of all states in the US 2 letter form.
///
/// ```
/// use us_state_info_rs::StateAbbr;
/// let iowa_state_abbr = StateAbbr::IA;
/// format!("{}", iowa_state_abbr);
///
/// ```
pub enum StateAbbr {
    AL,
    AK,
    AZ,
    AR,
    CA,
    CO,
    CT,
    DE,
    FL,
    GA,
    HI,
    ID,
    IL,
    IN,
    IA,
    KS,
    KY,
    LA,
    ME,
    MD,
    MA,
    MI,
    MN,
    MS,
    MO,
    MT,
    NE,
    NV,
    NH,
    NJ,
    NM,
    NY,
    NC,
    ND,
    OH,
    OK,
    OR,
    PA,
    RI,
    SC,
    SD,
    TN,
    TX,
    UT,
    VT,
    VA,
    WA,
    WV,
    WI,
    WY,
}

impl fmt::Display for StateAbbr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StateAbbr::AL => write!(f, "AL"),
            StateAbbr::AK => write!(f, "AK"),
            StateAbbr::AZ => write!(f, "AZ"),
            StateAbbr::AR => write!(f, "AR"),
            StateAbbr::CA => write!(f, "CA"),
            StateAbbr::CO => write!(f, "CO"),
            StateAbbr::CT => write!(f, "CT"),
            StateAbbr::DE => write!(f, "DE"),
            StateAbbr::FL => write!(f, "FL"),
            StateAbbr::GA => write!(f, "GA"),
            StateAbbr::HI => write!(f, "HI"),
            StateAbbr::ID => write!(f, "ID"),
            StateAbbr::IL => write!(f, "IL"),
            StateAbbr::IN => write!(f, "IN"),
            StateAbbr::IA => write!(f, "IA"),
            StateAbbr::KS => write!(f, "KS"),
            StateAbbr::KY => write!(f, "KY"),
            StateAbbr::LA => write!(f, "LA"),
            StateAbbr::ME => write!(f, "ME"),
            StateAbbr::MD => write!(f, "MD"),
            StateAbbr::MA => write!(f, "MA"),
            StateAbbr::MI => write!(f, "MI"),
            StateAbbr::MN => write!(f, "MN"),
            StateAbbr::MS => write!(f, "MS"),
            StateAbbr::MO => write!(f, "MO"),
            StateAbbr::MT => write!(f, "MT"),
            StateAbbr::NE => write!(f, "NE"),
            StateAbbr::NV => write!(f, "NV"),
            StateAbbr::NH => write!(f, "NH"),
            StateAbbr::NJ => write!(f, "NJ"),
            StateAbbr::NM => write!(f, "NM"),
            StateAbbr::NY => write!(f, "NY"),
            StateAbbr::NC => write!(f, "NC"),
            StateAbbr::ND => write!(f, "ND"),
            StateAbbr::OH => write!(f, "OH"),
            StateAbbr::OK => write!(f, "OK"),
            StateAbbr::OR => write!(f, "OR"),
            StateAbbr::PA => write!(f, "PA"),
            StateAbbr::RI => write!(f, "RI"),
            StateAbbr::SC => write!(f, "SC"),
            StateAbbr::SD => write!(f, "SD"),
            StateAbbr::TN => write!(f, "TN"),
            StateAbbr::TX => write!(f, "TX"),
            StateAbbr::UT => write!(f, "UT"),
            StateAbbr::VT => write!(f, "VT"),
            StateAbbr::VA => write!(f, "VA"),
            StateAbbr::WA => write!(f, "WA"),
            StateAbbr::WV => write!(f, "WV"),
            StateAbbr::WI => write!(f, "WI"),
            StateAbbr::WY => write!(f, "WY"),
        }
    }
}

#[derive(Debug)]
pub enum ParseStateAbbrError {
    InvalidAbbreviationError,
}

impl Display for ParseStateAbbrError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidAbbreviationError => write!(f, "invalid state abbreviation"),
        }
    }
}

impl Error for ParseStateAbbrError {}

impl FromStr for StateAbbr {
    type Err = ParseStateAbbrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_ascii_uppercase()[..] {
            "AL" => Ok(Self::AL),
            "AK" => Ok(Self::AK),
            "AZ" => Ok(Self::AZ),
            "AR" => Ok(Self::AR),
            "CA" => Ok(Self::CA),
            "CO" => Ok(Self::CO),
            "CT" => Ok(Self::CT),
            "DE" => Ok(Self::DE),
            "FL" => Ok(Self::FL),
            "GA" => Ok(Self::GA),
            "HI" => Ok(Self::HI),
            "ID" => Ok(Self::ID),
            "IL" => Ok(Self::IL),
            "IN" => Ok(Self::IN),
            "IA" => Ok(Self::IA),
            "KS" => Ok(Self::KS),
            "KY" => Ok(Self::KY),
            "LA" => Ok(Self::LA),
            "ME" => Ok(Self::ME),
            "MD" => Ok(Self::MD),
            "MA" => Ok(Self::MA),
            "MI" => Ok(Self::MI),
            "MN" => Ok(Self::MN),
            "MS" => Ok(Self::MS),
            "MO" => Ok(Self::MO),
            "MT" => Ok(Self::MT),
            "NE" => Ok(Self::NE),
            "NV" => Ok(Self::NV),
            "NH" => Ok(Self::NH),
            "NJ" => Ok(Self::NJ),
            "NM" => Ok(Self::NM),
            "NY" => Ok(Self::NY),
            "NC" => Ok(Self::NC),
            "ND" => Ok(Self::ND),
            "OH" => Ok(Self::OH),
            "OK" => Ok(Self::OK),
            "OR" => Ok(Self::OR),
            "PA" => Ok(Self::PA),
            "RI" => Ok(Self::RI),
            "SC" => Ok(Self::SC),
            "SD" => Ok(Self::SD),
            "TN" => Ok(Self::TN),
            "TX" => Ok(Self::TX),
            "UT" => Ok(Self::UT),
            "VT" => Ok(Self::VT),
            "VA" => Ok(Self::VA),
            "WA" => Ok(Self::WA),
            "WV" => Ok(Self::WV),
            "WI" => Ok(Self::WI),
            "WY" => Ok(Self::WY),
            _ => Err(ParseStateAbbrError::InvalidAbbreviationError),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::StateAbbr;

    #[test]
    fn test_iowa_display_impl() {
        let state_abbr = StateAbbr::IA;
        assert_eq!("IA", format!("{}", state_abbr))
    }

    #[test]
    fn test_colorado_display_impl() {
        let state_abbr = StateAbbr::CO;
        assert_eq!("CO", format!("{}", state_abbr))
    }
    #[test]
    fn test_iowa_fromstr_impl() {
        let state_abbr = "IA";
        assert_eq!(StateAbbr::IA, state_abbr.parse().unwrap());
        let state_abbr = "ia";
        assert_eq!(StateAbbr::IA, state_abbr.parse().unwrap());
    }

    #[test]
    fn test_colorado_fromstr_impl() {
        let state_abbr = "CO";
        assert_eq!(StateAbbr::CO, state_abbr.parse().unwrap());
        let state_abbr = "co";
        assert_eq!(StateAbbr::CO, state_abbr.parse().unwrap());
    }
}
