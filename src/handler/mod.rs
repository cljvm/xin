
use actix::prelude::*;
use actix_web::Error;
use db::DbExecutor;
use controller::Homepage;

impl Handler<Homepage> for DbExecutor {
    type Result = Result<String, Error>;
    fn handle(&mut self, msg: Homepage, _: &mut Self::Context) -> Self::Result {
        // let conn = self.conn().map_err("");
        

        Ok("done".to_owned())
    }
}