use schema::tags;

#[derive(Queryable)]
pub struct Tag {
    pub id: i32,
    pub label: String,
}

#[derive(Insertable)]
#[table_name="tags"]
pub struct NewTag<'a> {
    pub label: &'a str,
}
