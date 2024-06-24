pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

// could use an enum here as well but would need a match expression for every possible variant
impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // no interaction with state field
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self) // unwrap is okay because will never panic as Post always contains a Some value
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() { // take puts None in old state and moves out state
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    // all types need to implement this method
    // rather than self, &self or &mut self we use Box<Self> so only valid if called on a Box holding the type
    // taking ownership of the box invalidating the previous state
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // calling content on &Box<dyn State> deref coercision takes affect so content called on State trait
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        "" // default implementation so don't have to implement twice for Draft and PendingReview
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    // do nothing
    fn approve(self: Box<Self>) -> Box<dyn State> { // can't have default values with self as this violates object safety, macro might be better
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // do nothing
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}