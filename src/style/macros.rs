/// A macro to parse an identifier, or return an `OtherInvalidValue` error
/// otherwise.
///
/// This was originally taken from Servo and has been slightly modified since.
macro_rules! try_match_ident_ignore_ascii_case {
    ($input:expr, $( $match_body:tt )*) => {{
        let location = $input.current_source_location();
        let ident = $input.expect_ident_cloned()?;
        match_ignore_ascii_case! { &ident,
            $( $match_body )*
            _ => return Err(location.new_custom_error(
                crate::style::StyleParseErrorKind::OtherInvalidValue(ident.clone())
            ))
        }
    }}
}
