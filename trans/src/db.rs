use std::default::Default;

use sqlite3::{
    Access,
    DatabaseConnection,
    DatabaseUpdate,
    Query,
    ResultRowAccess,
    ResultRow,
    SqliteResult,
    ToSql,
};
use sqlite3::access;

pub struct Event {
    id: Option<i64>,
    pub date: u32,
    pub note: String,
    pub project_id: Option<i64>,
    pub text: String,
    //img: ,
    pub private: bool,
    pub num: f32,
}

#[deriving(Show, PartialEq)]
pub enum InputType {
    Num,
    Bool,
    Stack,
    Text,
    Img,
}


pub struct EventTemplate {
    id: Option<i64>,
    pub name: String,
    pub private: bool,
    pub input_type: InputType,
    //icon: Icon,
    pub mandatory: bool,
    pub tag_id: Option<i64>,
}

impl EventTemplate {
    pub fn new() -> EventTemplate {
        EventTemplate {
            id: None,
            name: "".into_string(),
            private: false,
            input_type: Num,
            mandatory: false,
            tag_id: None,
        }
    }
    pub fn id(&self) -> Option<i64> {self.id}
}

pub struct Tag {
    id: Option<i64>,
    pub name: String,
}

impl Tag {
    pub fn new(name: &str) -> Tag {
        Tag {
            id: None, 
            name: name.into_string(),
        }
    }

    fn from_db(id: i64, name: &str) -> Tag {
        Tag {
            id: Some(id), 
            name: name.into_string(),
        }
    }
    pub fn id(&self) -> Option<i64> {self.id}
}

pub struct Dao {
	conn: DatabaseConnection,
}

impl Dao {
	pub fn new() -> Dao {
		Dao {
			conn: match DatabaseConnection::new(access::ByFilename { flags: Default::default(), filename: "db.db" }) {
				Ok(c) => c,
				Err(e) => panic!(e),
			},
		}
	}

	pub fn load_tags(&mut self) -> Vec<Tag> {
		let mut stmt = match self.conn.prepare("SELECT id, name FROM tag") {
			Ok(s) => s,
			Err(e) => panic!(e),
		};
	    let mut data = vec!();
	    stmt.query(
	        [], |row| {
	        	let tag = Tag::from_db(row.column_int64(0), row.column_text(1).unwrap().as_slice());
	            data.push(tag);
	            Ok(())
	    });
	    data
	}

	pub fn persist_event_tags(&mut self, tags: &mut Vec<Tag>) {
		let mut update_stmt = match self.conn.prepare("UPDATE tag SET name = ? WHERE id = ?") {
			Ok(s) => s,
			Err(e) => panic!(e),
		};
		let mut insert_stmt = match self.conn.prepare("INSERT INTO tag(name) VALUES (?)") {
			Ok(s) => s,
			Err(e) => panic!(e),
		};
		for tag in tags.iter_mut() {
			if tag.id.is_none() {
				match self.conn.update(&mut insert_stmt, [&tag.name as &ToSql]) {
					Err(e) => panic!(e),
					_ => {},
				};
				insert_stmt.clear_bindings();
				tag.id = Some(self.conn.last_insert_rowid());
			} else {
				println!("update tag: {}, {}", tag.id, tag.name);
				match self.conn.update(&mut update_stmt, [&tag.name as &ToSql, &tag.id as &ToSql])  {
					Err(e) => panic!(e),
					_ => {},
				};
				update_stmt.clear_bindings();
			}
		}
	}

	pub fn load_event_templates(&mut self) -> Vec<EventTemplate> {
		let mut stmt = match self.conn.prepare("SELECT id, input_type, private, mandatory, name, tag_id FROM event_template") {
			Ok(s) => s,
			Err(e) => panic!(e),
		};
	    let mut data = vec!();
	    stmt.query(
	        [], |row| {
	        	let template = EventTemplate {
		            id: Some(row.column_int64(0)),
		            input_type: match row.column_text(1).unwrap().as_slice() {
		            	"Bool" => Bool,
		            	"Stack" => Stack,
		            	"Text" => Text,
		            	"Img" => Img,
		            	_ => Num,
		            },
		            private: row.column_int(2) == 1,
		            mandatory: row.column_int(3) == 1,
		            name: row.column_text(4).unwrap(),
		            tag_id: Some(row.column_int64(5)),
		        };
	            data.push(template);
	            Ok(())
	    });
	    data
	}

	pub fn persist_event_templates(&mut self, temps: &mut Vec<EventTemplate>) {
		let mut update_stmt = match self.conn.prepare("UPDATE event_template SET input_type = ?, name = ?,  private = ?, mandatory = ?, tag_id = ? WHERE id = ?") {
			Ok(s) => s,
			Err(e) => panic!(e),
		};
		let mut insert_stmt = match self.conn.prepare("INSERT INTO event_template(input_type, name, private, mandatory, tag_id) VALUES (?, ?, ?, ?, ?)") {
			Ok(s) => s,
			Err(e) => panic!(e),
		};
		for temp in temps.iter_mut() {
			if temp.id.is_none() {
				println!("{}", temp.tag_id);
				match self.conn.update(&mut insert_stmt, 
					[	
						&temp.input_type.to_string()  as &ToSql,
						&temp.name as &ToSql,
						&(temp.private as i32)  as &ToSql,
						&(temp.mandatory as i32) as &ToSql,
						&temp.tag_id as &ToSql,
					]) {
					Err(e) => panic!(e),
					_ => {},
				};
				insert_stmt.clear_bindings();
				temp.id = Some(self.conn.last_insert_rowid());
			} else {
				match self.conn.update(&mut update_stmt, 
					[	
						&temp.input_type.to_string() as &ToSql,
						&temp.name as &ToSql,
						&(temp.private as i32) as &ToSql,
						&(temp.mandatory as i32) as &ToSql,
						&temp.tag_id as &ToSql,
						&temp.id as &ToSql,
					]) {
					Err(e) => panic!(e),
					_ => {},
				};
				update_stmt.clear_bindings();
			}
		}
	}
}