use nom::Err;

use crate::parser::parser_error::ParserError;


#[macro_export]
macro_rules! convert_parser_data_error {
    ($self:expr, $data_type:expr) => {
        $self.map_err(|e: Err<ParserError<&str>>| {
            match e {
                Error(ParserError::Nom(i, _ek)) => {
                    if i.is_empty() {
                        Error(ParserError::Incomplete)
                    } else {
                        Error(ParserError::DataError($data_type))
                    }
                },
                Failure(ParserError::Nom(i, _ek)) => {
                    if i.is_empty() {
                        Failure(ParserError::Incomplete)
                    } else {
                        Failure(ParserError::DataError($data_type))
                    } 
                },
                e => e,
            }
        })
    };
}

pub fn inner_nom_err<I>(err: Err<ParserError<I>>) -> ParserError<I>{
    match err {
        Err::Error(e) => e,
        Err::Failure(e) => e,
        //Incomplete is never used in code
        Err::Incomplete(e) => ParserError::Unknown(String::from("Incomplete Parsing")),
    }
}