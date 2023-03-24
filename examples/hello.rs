#[derive(Debug)]
struct Fruit(i32);

impl From<i32> for Fruit {
    fn from(a: i32) -> Self {
        Fruit(a)
    }
}

fn eat(a: Fruit) {
    println!("Eat number {:?}", a);
}

fn main() {
    println!("Hello from an example!");
    let fruit_a = 1;
    eat(fruit_a.into());
    // eat2(fruit_a);

    // request
    // routes -> handle

    // handles

    // .route("/subscriptions", web::post().to(subscribe))
    // pub async fn subscribe(
    //     form: web::Form<FormData>,
    //     pool: web::Data<PgPool>,
    //     email_client: web::Data<EmailClient>,
    //     base_url: web::Data<ApplicationBaseUrl>,
    //     // ) -> Result<HttpResponse, actix_web::Error> {
    // ) -> Result<HttpResponse, SubscribeError> {

    // .route("/dashboard", web::get().to(admin_dashboard))
    // .wrap(from_fn(reject_anonymous_users))
    // pub async fn admin_dashboard(
    //     pool: web::Data<PgPool>,
    //     user_id: web::ReqData<UserId>,
    // )

    // .route("/logout", web::post().to(log_out)),
    // pub async fn log_out(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    // reu
}

// fn post() {
//     // a

//     log_out(request, payload)
// }

type request = i32;

#[derive(Debug)]
struct Fruit2(request);
impl Fruit2 {
    pub fn from_request(a: request) -> Self {
        Fruit2(a)
    }
}

impl From<request> for Fruit2 {
    fn from(a: request) -> Self {
        Self::from_request(a)
        // Fruit2(a)
    }
}
