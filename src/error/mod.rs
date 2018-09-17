
pub struct Error {

}

#[derive(Serialize)]
pub struct ErrorMessage {
    pub code: i32,
    pub msg: String,
    pub detail: Option<String>,
}

impl Error {
    pub fn database_connection_get_error() {
        
    }
}