use ic_cdk::api::time;
use ic_cdk_macros::*;
use candid::{CandidType, Deserialize, Principal};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
struct Post {
    id: u64,
    content: String,
    author: Principal,
    timestamp: u64,
    likes: Vec<Principal>,
}

thread_local! {
    static POSTS: RefCell<HashMap<u64, Post>> = RefCell::new(HashMap::new());
    static NEXT_ID: RefCell<u64> = RefCell::new(0);
}

#[update]
fn create_post(content: String) -> Post {
    let author = ic_cdk::caller();
    let timestamp = time();

    let post = POSTS.with(|posts| {
        let mut posts = posts.borrow_mut();
        let id = NEXT_ID.with(|next| {
            let id = *next.borrow();
            *next.borrow_mut() += 1;
            id
        });

        let post = Post {
            id,
            content,
            author,
            timestamp,
            likes: vec![],
        };

        posts.insert(id, post.clone());
        post
    });

    post
}

#[query]
fn get_all_posts() -> Vec<Post> {
    POSTS.with(|posts| posts.borrow().values().cloned().collect())
}
