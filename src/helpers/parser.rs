
#[macro_export]
macro_rules! convert_parser_error {
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
                e => e,
            }
        })
    };
}