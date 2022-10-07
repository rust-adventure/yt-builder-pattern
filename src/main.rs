use yt_builder_pattern::BlogPost;

fn main() {
    let mut post =
        BlogPost::new("Builder APIs in Rust".to_string());

    post.tag("rust").tag("design-pattern");

    post.tag("rust");

    post.slug("builder-apis");

    let final_post =
        post.post("do this, then that".to_string());

    println!("{}", final_post.as_file());
}
