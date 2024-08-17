use self::models::*;
use diesel::prelude::*;
use suiwei::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = memories
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading memories");

    println!("Displaying {} memories", results.len());
    for memory in results {
        println!("{}", memory.image);
        println!("-----------\n");
        println!("{}", memory.description);
    }
}
