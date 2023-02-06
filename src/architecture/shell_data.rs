
pub enum ShellData {
    FilePath {value: String},
    DirPath {value: String},
    GlobPath {value: String},
    String {value: String},
    Number {value: f64},
    Float {value: f64},
    Int {value: i64},
    Nothing,
}
