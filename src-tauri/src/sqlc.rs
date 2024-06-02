use serde::{Deserialize, Serialize};
use sqlite::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateInfo  {
    id: String,
    name: String,
    parentId: String,
    body: String,
}

pub fn connect(path: &str) -> Result<Connection> {
    let connection: Result<Connection, > = sqlite::open(path);
    return connection;
}

pub fn init(connection: Connection) {

    let query: &str = "
    CREATE TABLE Templates (id TEXT, name TEXT, body TEXT, parentId TEXT);
    INSERT INTO Templates VALUES ('test', 'test1', 'aaaaaaaaaaa', '');
    INSERT INTO Templates VALUES ('test2', 'test2', 'bbbbbBbbbbbb', '');
    ";

    connection.execute(query);
}

pub fn write_template(connection: Connection, template_info: TemplateInfo, isUpdate: bool) -> Result<String> {

    // let a: Result<String, Error>;

    if (!isUpdate) {
        return insert_template(connection, template_info);
    }

    if (isUpdate) {
        return update_template(connection, template_info);
    }

    Ok("test".to_string())
}


pub fn insert_template(connection: Connection, template_info: TemplateInfo) -> Result<String> {

    let query = "INSERT INTO Templates (id,name,body,parentId) VALUES (:id,:name,:body,:parentId)";
    let mut statement: Statement = connection.prepare(query).unwrap();

    println!("insert!");

    statement.bind_iter::<_, (_, Value)>([
        (":name", template_info.name.as_str().into()),
        (":body", template_info.body.as_str().into()),
        (":parentId", template_info.parentId.as_str().into()),
        (":id", template_info.id.as_str().into())
    ])?;

    while let Ok(State::Row) = statement.next() {
        println!("OK");
    }

    Ok("test".to_string())
}   

pub fn update_template(connection: Connection, template_info: TemplateInfo) -> Result<String> {

    let query = "UPDATE Templates SET name=:name,body=:body,parentId=:parentId WHERE id=:id";
    let mut statement: Statement = connection.prepare(query).unwrap();

    statement.bind_iter::<_, (_, Value)>([
        (":name", template_info.name.as_str().into()),
        (":body", template_info.body.as_str().into()),
        (":parentId", template_info.parentId.as_str().into()),
        (":id", template_info.id.as_str().into())
    ])?;

    while let Ok(State::Row) = statement.next() {
        println!("OK");
    }

    Ok("test".to_string())
}   

pub fn get_templates(connection: Connection) -> Vec<TemplateInfo> {

    let query = "SELECT id,name,parentId,body FROM Templates;";
    let mut statement: Statement = connection.prepare(query).unwrap();

    let mut v: Vec<TemplateInfo> = vec![];

    while let Ok(State::Row) = statement.next() {
        
        let id = statement.read::<String, _>("id").unwrap().to_string();
        let name = statement.read::<String, _>("name").unwrap();
        let parent_id = statement.read::<String, _>("parentId").unwrap();
        let body: String = statement.read::<String, _>("body").unwrap();

        v.push(TemplateInfo { id, name, parentId: parent_id, body });
    }

    return v;

    // statement.bind((1, 50)).unwrap();
    // prepare(connection, statement);
}

pub fn prepare(connection: Connection, mut statement: Statement) -> Statement {

    // let query = "SELECT * FROM users WHERE age > ?";

    while let Ok(State::Row) = statement.next() {
        println!("name = {}", statement.read::<String, _>("name").unwrap());
        println!("age = {}", statement.read::<i64, _>("age").unwrap());
    }

    return statement;
}