use super::{Version, VersionState};
use anyhow::Result;
use nom::bytes::complete::*;
use nom::character::complete::char;
use nom::combinator::{map_res, opt};
use nom::error::{Error, ErrorKind};
use nom::{IResult, Parser};

fn number(s: &str) -> IResult<&str, usize> {
    map_res(take_while(|v: char| v.is_ascii_digit()), |v: &str| {
        v.parse::<usize>()
    })
    .parse(s)
}

fn prefix<'a>(p: &'a str, s: &'a str) -> IResult<&'a str, &'a str> {
    tag(p)(s)
}

fn delim(s: &str) -> IResult<&str, char> {
    char('.')(s)
}

fn suffix_version(s: &str) -> IResult<&str, &str> {
    let (s, _) = opt(char('.')).parse(s)?;
    take_while(|c: char| c.is_ascii()).parse(s)
}

fn suffix(s: &str) -> IResult<&str, VersionState> {
    let (s, _) = opt(char('-')).parse(s)?;
    if let Ok((s, _)) = prefix("alpha", s) {
        return suffix_version(s).map(|(s, v)| (s, VersionState::Alpha(v.into())));
    };
    if let Ok((s, _)) = prefix("beta", s) {
        return suffix_version(s).map(|(s, v)| (s, VersionState::Beta(v.into())));
    };
    if let Ok((s, _)) = prefix("rc", s) {
        return suffix_version(s).map(|(s, v)| (s, VersionState::ReleaseCandidate(v.into())));
    };
    Err(nom::Err::Failure(nom::error::Error {
        input: s,
        code: ErrorKind::Fail,
    }))
}

fn version_prefix(s: &str) -> IResult<&str, ()> {
    char::<&str, Error<_>>('v')(s)
        .or_else(|_| char::<&str, Error<_>>('V')(s))
        .map(|(s, _)| (s, ()))
        .or(Ok((s, ())))
}

/// Takes a string and tries to parse it into a [`Version`].
pub fn parse_version(s: &str) -> Result<Version> {
    let mut version = Version::default();

    let (s, _) =
        version_prefix(s).map_err(|e| anyhow::anyhow!("failed parsing version prefix: {e}"))?;

    let (s, major) = number(s).map_err(|e| anyhow::anyhow!("failed parsing major version: {e}"))?;
    version.major = major;

    if s.is_empty() {
        return Ok(version);
    }

    let Ok((s, _)) = delim(s) else {
        let (_, suffix) = suffix(s).map_err(|e| anyhow::anyhow!("failed parsing suffix: {e}"))?;
        version.pre = Some(suffix);
        return Ok(version);
    };

    let (s, minor) = number(s).map_err(|e| anyhow::anyhow!("failed parsing minor version: {e}"))?;
    version.minor = Some(minor);

    if s.is_empty() {
        return Ok(version);
    }

    let Ok((s, _)) = delim(s) else {
        let (_, suffix) = suffix(s).map_err(|e| anyhow::anyhow!("failed parsing suffix: {e}"))?;
        version.pre = Some(suffix);
        return Ok(version);
    };

    let (s, patch) = number(s).map_err(|e| anyhow::anyhow!("failed parsing patch version: {e}"))?;
    version.patch = Some(patch);

    if s.is_empty() {
        return Ok(version);
    }

    let (_, suffix) = suffix(s).map_err(|e| anyhow::anyhow!("failed parsing suffix: {e}"))?;
    version.pre = Some(suffix);

    Ok(version)
}
