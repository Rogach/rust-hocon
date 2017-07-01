use nom;

quick_error! {
    #[derive(Debug, PartialEq)]
    pub enum Error {
        ExtraInput(i: isize) {
            description("extra input was left after parsing")
        }
        IncompleteInput(err: nom::Needed) {
            description("not enough input to parse")
        }
        ParseError(err: String) {
            description("error during parsing")
        }
        PathError(path: String) {
            description("incorrect path expression")
        }
        NotFound(path: String) {
            description("path not found in config")
        }
    }
}
