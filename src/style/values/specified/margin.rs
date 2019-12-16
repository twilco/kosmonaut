use crate::style::values::specified::LengthPercentage;

/// Specified values for `margin-bottom`.
///
/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
pub struct MarginBottom {
    // TODO: This should also support the `auto` keyword
    pub length_percentage: LengthPercentage
}

/// Specified values for `margin-left`.
///
/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
pub struct MarginLeft {
    // TODO: This should also support the `auto` keyword
    pub length_percentage: LengthPercentage
}

/// Specified values for `margin-right`.
///
/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
pub struct MarginRight {
    // TODO: This should also support the `auto` keyword
    pub length_percentage: LengthPercentage
}

/// Specified values for `margin-top`.
///
/// https://www.w3.org/TR/2018/WD-css-box-3-20181218/#property-index
pub struct MarginTop {
    // TODO: This should also support the `auto` keyword
    pub length_percentage: LengthPercentage
}

