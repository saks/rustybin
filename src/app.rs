use rocket_contrib::Template;
use rocket::http::RawStr;

#[derive(Serialize)]
struct IndexContext<'a> {
    name: &'a str,
    items: Vec<&'a str>,
}

#[get("/<name>")]
fn index(name: &RawStr) -> Template {
    // format!("{}", req)
    // let mut context: HashMap<&str, &str> = HashMap::new();
    // context.insert("name", "Alex");
    let context = IndexContext {
        name: name,
        items: vec!["foo", "bar"],
    };
    Template::render("index", &context)
}
