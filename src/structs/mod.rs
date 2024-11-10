/*
    Structures for better control over
    types and for the safety of inserts
    in Database
*/
pub struct InsertMessage {
    pub id: i32,
    pub message: String,
}
