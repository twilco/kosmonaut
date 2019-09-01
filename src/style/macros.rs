/// A macro to parse an identifier, or return an `UnexpectedIdent` error
/// otherwise.
///
/// This was taken from Servo.
macro_rules! try_match_ident_ignore_ascii_case {
    ($input:expr, $( $match_body:tt )*) => {{
        let location = $input.current_source_location();
        let ident = $input.expect_ident_cloned()?;
        match_ignore_ascii_case! { &ident,
            $( $match_body )*
            _ => return Err(location.new_custom_error(
                ::selectors::parser::SelectorParseErrorKind::UnexpectedIdent(ident.clone())
            ))
        }
    }}
}
