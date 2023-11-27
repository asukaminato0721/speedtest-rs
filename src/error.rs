#[derive(Debug)]
pub enum SpeedTestError {
    Reqwest(reqwest::Error),
    Io(::std::io::Error),
    Csv(csv::Error),
    Json(serde_json::Error),
    ParseFloatError(std::num::ParseFloatError),
    ParseIntError(std::num::ParseIntError),
    AddrParseError(std::net::AddrParseError),
    RoXmlTreeError(roxmltree::Error),
    ConfigParseError,
    ServerParseError,
    LatencyTestInvalidPath,
    LatencyTestClosestError,
    UrlParseError(url::ParseError),
    SystemTimeError(std::time::SystemTimeError),
    ParseShareUrlError,
    ThreadPoolBuildError(rayon::ThreadPoolBuildError),
}

impl From<reqwest::Error> for SpeedTestError {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl From<::std::io::Error> for SpeedTestError {
    fn from(err: ::std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<csv::Error> for SpeedTestError {
    fn from(err: csv::Error) -> Self {
        Self::Csv(err)
    }
}

impl From<serde_json::Error> for SpeedTestError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

impl From<std::num::ParseFloatError> for SpeedTestError {
    fn from(err: std::num::ParseFloatError) -> Self {
        Self::ParseFloatError(err)
    }
}

impl From<std::num::ParseIntError> for SpeedTestError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}

impl From<std::net::AddrParseError> for SpeedTestError {
    fn from(err: std::net::AddrParseError) -> Self {
        Self::AddrParseError(err)
    }
}

impl From<roxmltree::Error> for SpeedTestError {
    fn from(err: roxmltree::Error) -> Self {
        Self::RoXmlTreeError(err)
    }
}

impl From<url::ParseError> for SpeedTestError {
    fn from(err: url::ParseError) -> Self {
        Self::UrlParseError(err)
    }
}

impl From<std::time::SystemTimeError> for SpeedTestError {
    fn from(err: std::time::SystemTimeError) -> Self {
        Self::SystemTimeError(err)
    }
}

impl From<rayon::ThreadPoolBuildError> for SpeedTestError {
    fn from(err: rayon::ThreadPoolBuildError) -> Self {
        Self::ThreadPoolBuildError(err)
    }
}
