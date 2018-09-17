#[macro_export]
macro_rules! send_msg {
    ($act:expr, $msg:expr) => {
        send_msg!($act, $msg, |s| Ok(HttpResponse::Ok().json(s)))
    };
    ($act:expr, $msg:expr, $fn:expr) => {
        $act.send($msg)
            .from_err::<actix::MailboxError>()
            .and_then(|res| match res {
                Ok(s) => $fn(s),
                Err(_) => Ok(HttpResponse::InternalServerError().into()),
            }).responder()
    };
}
