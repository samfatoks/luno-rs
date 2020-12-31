use crate::error::Error;
use std::{fmt, str};
/// Currency pairs available on Luno.
///
/// # Examples
///
/// ```
/// use luno::CurrencyPair;
/// let currency_pair = CurrencyPair::default();
/// assert_eq!(currency_pair, CurrencyPair::XBTNGN);
/// ```
/// Strings can also be converted to CurrencyPair easily.
/// ```
/// use luno::CurrencyPair;
/// let currency_pair: CurrencyPair = "ETHNGN".parse().unwrap();
/// assert_eq!(currency_pair, CurrencyPair::ETHNGN);
/// ```
///
/// # Error
///
/// Error::InvalidCurrencyPair is returned if string cannot to converted to a valid currency pair.
#[derive(Debug, PartialEq)]
pub enum CurrencyPair {
    BCHXBT,
    XBTAUD,
    XBTEUR,
    XBTGBP,
    XBTIDR,
    XBTMYR,
    XBTNGN,
    XBTSGD,
    XBTUGX,
    XBTZAR,
    XBTZMW,
    ETHAUD,
    ETHXBT,
    ETHEUR,
    ETHGBP,
    ETHIDR,
    ETHMYR,
    ETHNGN,
    ETHZAR,
    LTCXBT,
    LTCMYR,
    LTCNGN,
    LTCZAR,
    XRPXBT,
    XRPMYR,
    XRPNGN,
    XRPZAR,
}

impl fmt::Display for CurrencyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CurrencyPair::BCHXBT => write!(f, "BCHXBT"),
            CurrencyPair::XBTAUD => write!(f, "XBTAUD"),
            CurrencyPair::XBTEUR => write!(f, "XBTEUR"),
            CurrencyPair::XBTGBP => write!(f, "XBTGBP"),
            CurrencyPair::XBTIDR => write!(f, "XBTIDR"),
            CurrencyPair::XBTMYR => write!(f, "XBTMYR"),
            CurrencyPair::XBTNGN => write!(f, "XBTNGN"),
            CurrencyPair::XBTSGD => write!(f, "XBTSGD"),
            CurrencyPair::XBTUGX => write!(f, "XBTUGX"),
            CurrencyPair::XBTZAR => write!(f, "XBTZAR"),
            CurrencyPair::XBTZMW => write!(f, "XBTZMW"),
            CurrencyPair::ETHAUD => write!(f, "ETHAUD"),
            CurrencyPair::ETHXBT => write!(f, "ETHXBT"),
            CurrencyPair::ETHEUR => write!(f, "ETHEUR"),
            CurrencyPair::ETHGBP => write!(f, "ETHGBP"),
            CurrencyPair::ETHIDR => write!(f, "ETHIDR"),
            CurrencyPair::ETHMYR => write!(f, "ETHMYR"),
            CurrencyPair::ETHNGN => write!(f, "ETHNGN"),
            CurrencyPair::ETHZAR => write!(f, "ETHZAR"),
            CurrencyPair::LTCXBT => write!(f, "LTCXBT"),
            CurrencyPair::LTCMYR => write!(f, "LTCMYR"),
            CurrencyPair::LTCNGN => write!(f, "LTCNGN"),
            CurrencyPair::LTCZAR => write!(f, "LTCZAR"),
            CurrencyPair::XRPXBT => write!(f, "XRPXBT"),
            CurrencyPair::XRPMYR => write!(f, "XRPMYR"),
            CurrencyPair::XRPNGN => write!(f, "XRPNGN"),
            CurrencyPair::XRPZAR => write!(f, "XRPZAR"),
        }
    }
}

impl str::FromStr for CurrencyPair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BCHXBT" => Ok(CurrencyPair::BCHXBT),
            "XBTAUD" => Ok(CurrencyPair::XBTAUD),
            "XBTEUR" => Ok(CurrencyPair::XBTEUR),
            "XBTGBP" => Ok(CurrencyPair::XBTGBP),
            "XBTIDR" => Ok(CurrencyPair::XBTIDR),
            "XBTMYR" => Ok(CurrencyPair::XBTMYR),
            "XBTNGN" => Ok(CurrencyPair::XBTNGN),
            "XBTSGD" => Ok(CurrencyPair::XBTSGD),
            "XBTUGX" => Ok(CurrencyPair::XBTUGX),
            "XBTZAR" => Ok(CurrencyPair::XBTZAR),
            "XBTZMW" => Ok(CurrencyPair::XBTZMW),
            "ETHAUD" => Ok(CurrencyPair::ETHAUD),
            "ETHXBT" => Ok(CurrencyPair::ETHXBT),
            "ETHEUR" => Ok(CurrencyPair::ETHEUR),
            "ETHGBP" => Ok(CurrencyPair::ETHGBP),
            "ETHIDR" => Ok(CurrencyPair::ETHIDR),
            "ETHMYR" => Ok(CurrencyPair::ETHMYR),
            "ETHNGN" => Ok(CurrencyPair::ETHNGN),
            "ETHZAR" => Ok(CurrencyPair::ETHZAR),
            "LTCXBT" => Ok(CurrencyPair::LTCXBT),
            "LTCMYR" => Ok(CurrencyPair::LTCMYR),
            "LTCNGN" => Ok(CurrencyPair::LTCNGN),
            "LTCZAR" => Ok(CurrencyPair::LTCZAR),
            "XRPXBT" => Ok(CurrencyPair::XRPXBT),
            "XRPMYR" => Ok(CurrencyPair::XRPMYR),
            "XRPNGN" => Ok(CurrencyPair::XRPNGN),
            "XRPZAR" => Ok(CurrencyPair::XRPZAR),
            _ => Err(Error::InvalidCurrencyPair(s.to_string())),
        }
    }
}

impl Default for CurrencyPair {
    fn default() -> Self {
        CurrencyPair::XBTNGN
    }
}

#[cfg(test)]
mod tests {
    use super::CurrencyPair;

    #[test]
    fn test_currency_pair_to_valid_conversion() {
        assert_eq!("XBTNGN", CurrencyPair::XBTNGN.to_string());
    }

    #[test]
    fn test_string_to_valid_currency_pair_conversion() {
        let cp: CurrencyPair = "XBTNGN".parse().unwrap();
        assert_eq!(CurrencyPair::XBTNGN, cp);
    }

    #[test]
    fn test_panic_when_string_is_invalid_currency_pair() {
        assert!("XBTLUNO".parse::<CurrencyPair>().is_err());
    }

    #[test]
    fn test_default_currency_pair() {
        let currency_pair = CurrencyPair::default();
        assert_eq!(currency_pair, CurrencyPair::XBTNGN);
    }
}
