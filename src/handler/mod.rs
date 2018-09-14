


impl Handler<Ping> for DbExecutor {
    type Result = Result<Vec<Demo>, Error>;
    fn handle(&mut self, msg: Ping, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get().unwrap();
        let mut vec = Vec::new();
        for row in &conn.query("SELECT id, name FROM lemon_demo WHERE name = $1", &[&msg.0]).unwrap() {
            let d = Demo {
                id: row.get(0),
                name: row.get(1),
            };
            println!("id:{},name:{}", d.id, d.name);
            vec.push(d);
        }

        // diesel::insert_into(users)
        //     .values(&new_user)
        //     .execute(conn)
        //     .map_err(|_| error::ErrorInternalServerError("Error inserting person"))?;

        Ok(vec)
    }
}