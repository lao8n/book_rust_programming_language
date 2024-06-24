use state_pattern_types::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review(); // return new instances so shadow with let post

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}