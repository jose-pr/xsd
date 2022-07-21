use crate::_internal::*;


#[derive(Debug)]
pub enum WhiteSpace {
    ///No normalization is performed; the value is not changed for element content as required by the W3C XML 1.0 Recommendation.
    Preserved,
    ///All occurrences of #x9 (tab), #xA (line feed) and #xD (carriage return) are replaced with #x20 (space).
    Replace,
    ///After the processing implied by replace, contiguous sequences of #x20s are collapsed to a single #x20, and leading and trailing #x20s are removed.
    Collapsed,
}
#[derive(Debug)]
pub enum Facet {
    ///Number of units of length. Units of length depend on the data type.
    Length(usize),
    ///Minimum number of units of length. Units of length depend on the data type.
    MinLength(usize),
    ///Maximum number of units of length. Units of length depend on the data type.
    MaxLength(usize),
    ///Specific pattern that the data type's values must match. This constrains the data type to literals that match the specified pattern. The pattern value must be a regular expression.
    Pattern(Str),
    ///This constrains a data type to the specified values.
    Enumeration(List<Str>),
    ///The whiteSpace facet cannot be changed for most numeric data types
    WhiteSpace(WhiteSpace),
    ///Value with specific maximum number of decimal digits
    TotalDigits(usize),
    ///Value with specific maximum number of decimal digits in the fractional part.
    FractionDigits(usize),
    ///Minimum value. This value must be the same data type as the inherited data type.
    MinInclusive(Str),
    ///Maximum value. This value must be the same data type as the inherited data type.
    MaxInclusive(Str),
    ///Lower bound value (all values are greater than this value). This value must be the same data type as the inherited data type.
    MinExclusive(Str),
    ///Upper bound value (all values are less than this value). This value must be the same data type as the inherited data type.
    MaxExclusive(Str),
}
#[derive(Debug)]
pub enum Primitive {
    //length, pattern, maxLength, minLength, enumeration, whiteSpace
    ///Represents character strings.
    String,

    //pattern, whiteSpace
    ///Represents Boolean values, which are either true or false.
    Boolean,

    //enumeration, pattern, totalDigits, fractionDigits, minInclusive, maxInclusive, maxExclusive, whiteSpace
    ///Represents arbitrary precision numbers.
    Decimal,

    // pattern, enumeration, minInclusive, minExclusive, maxInclusive, maxExclusive, whiteSpace
    ///Represents single-precision 32-bit floating-point numbers.
    Float,

    //  pattern, enumeration, minInclusive, minExclusive, maxInclusive, maxExclusive, whiteSpace
    ///Represents double-precision 64-bit floating-point numbers.
    Double,

    // enumeration, pattern, minInclusive, minExclusive, maxInclusive, maxExclusive, whiteSpace
    ///Represents a duration of time.
    //    The pattern for duration is PnYnMnDTnHnMnS, where nY represents the number of years, nM the number of months, nD the number of days, T the date/time separator, nH the number of hours, nM the number of minutes, and nS the number of seconds.
    Duration,

    //enumeration, pattern, minInclusive, minExclusive, maxInclusive, maxExclusive, whiteSpace
    ///Represents a specific instance of time.
    //The pattern for dateTime is CCYY-MM-DDThh:mm:ss where CC represents the century, YY the year, MM the month, and DD the day, preceded by an optional leading negative (-) character to indicate a negative number. If the negative character is omitted, positive (+) is assumed. The T is the date/time separator and hh, mm, and ss represent hour, minute, and second respectively. Additional digits can be used to increase the precision of fractional seconds if desired. For example, the format ss.ss... with any number of digits after the decimal point is supported. The fractional seconds part is optional.

    //This representation may be immediately followed by a "Z" to indicate Coordinated Universal Time (UTC) or to indicate the time zone. For example, the difference between the local time and Coordinated Universal Time, immediately followed by a sign, + or -, followed by the difference from UTC represented as hh:mm (minutes is required). If the time zone is included, both hours and minutes must be present.
    DateTime,

    // enumeration, pattern, minInclusive, minExclusive, maxInclusive, maxExclusive, whiteSpace
    ///Represents an instance of time that recurs every day.
    //The pattern for time is hh:mm:ss.sss with optional time zone indicator.
    Time,

    //enumeration, pattern, minInclusive, minExclusive, maxInclusive, maxExclusive, whiteSpace
    ///Represents a calendar date.
    // The pattern for date is CCYY-MM-DD with optional time zone indicator as allowed for dateTime.
    Date,

    //enumeration, pattern, minInclusive, minExclusive, maxInclusive, maxExclusive, whiteSpace
    ///Represents a specific Gregorian month in a specific Gregorian year. A set of one-month long, nonperiodic instances.
    //The pattern for gYearMonth is CCYY-MM with optional time zone indicator.
    GregorianYearMonth,

    //enumeration, pattern, minInclusive, minExclusive, maxInclusive, maxExclusive, whiteSpace
    ///Represents a Gregorian year. A set of one-year long, nonperiodic instances.
    //The pattern for gYear is CCYY with optional time zone indicator as allowed for dateTime.
    GYear,

    //enumeration, pattern, minInclusive, minExclusive, maxInclusive, maxExclusive, whiteSpace
    ///Represents a specific Gregorian date that recurs, specifically a day of the year such as the third of May. A gMonthDay is the set of calendar dates. Specifically, it is a set of one-day long, annually periodic instances.
    //The pattern for gMonthDay is --MM-DD with optional time zone indicator as allowed for date.
    GregorianMonthDay,

    //enumeration, pattern, minInclusive, minExclusive, maxInclusive, maxExclusive, whiteSpace
    ///Represents a Gregorian day that recurs, specifically a day of the month such as the fifth day of the month. A gDay is the space of a set of calendar dates. Specifically, it is a set of one-day long, monthly periodic instances.
    //  The pattern for gDay is ---DD with optional time zone indicator as allowed for date.
    GregorianDay,

    // enumeration, pattern, minInclusive, minExclusive, maxInclusive, maxExclusive, whiteSpace
    ///  Represents a Gregorian month that recurs every year. A gMonth is the space of a set of calendar months. Specifically, it is a set of one-month long, yearly periodic instances.
    // The pattern for gMonth is --MM-- with optional time zone indicator as allowed for date.
    GregorianMonth,

    //  length, pattern, maxLength, minLength, enumeration, whiteSpace
    ///Represents arbitrary hex-encoded binary data. A hexBinary is the set of finite-length sequences of binary octets. Each binary octet is encoded as a character tuple, consisting of two hexadecimal digits ([0-9a-fA-F]) representing the octet code.
    HexBinary,

    //length, pattern, maxLength, minLength, enumeration, whiteSpace
    ///Represents Base64-encoded arbitrary binary data. A base64Binary is the set of finite-length sequences of binary octets.
    Base64Binary,

    //length, pattern, maxLength, minLength, enumeration, whiteSpace
    ///Represents a URI as defined by RFC 2396. An anyURI value can be absolute or relative, and may have an optional fragment identifier.
    AnyURI,

    //  length, enumeration, pattern, maxLength, minLength, whiteSpace
    ///Represents a qualified name. A qualified name is composed of a prefix and a local name separated by a colon. Both the prefix and local names must be an NCName. The prefix must be associated with a namespace URI reference, using a namespace declaration.
    QName,

    //  length, enumeration, pattern, maxLength, minLength, whiteSpace
    ///   Represents a NOTATION attribute type. A set of QNames.
    Notation,
}
