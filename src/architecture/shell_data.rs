
#[derive(Debug, Clone, PartialEq)]
pub enum ShellData {
    FilePath {value: String},
    DirPath {value: String},
    GlobPath {value: String},
    String {value: String},
    Float {value: f32},
    Int {value: i32},
    Nothing,
}
