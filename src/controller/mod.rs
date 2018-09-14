


fn index((name, state): (Path<String>, State<AppState>), ) -> FutureResponse<HttpResponse> {
    // send async `CreateUser` message to a `DbExecutor`
    state
        .db
        .send(Ping(name.into_inner()))
        .from_err()
        .and_then(|res| match res {
            Ok(s) => Ok(HttpResponse::Ok().json(s)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}