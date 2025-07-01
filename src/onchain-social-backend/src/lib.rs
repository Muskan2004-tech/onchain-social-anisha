use ic_cdk::api::time;
use ic_cdk_macros::*;
use candid::{CandidType, Deserialize, Principal};
use std::cell::RefCell;
use std::collections::HashMap;

const DEFAULT_AVATAR_URL: &str = "https://i.ibb.co/Pr3bY4X/default-avatar.png"; //Step 1: Default avatar

#[derive(CandidType, Deserialize, Clone)]
struct Post {
    id: u64,
    content: String,
    author: Principal,
    timestamp: u64,
    likes: Vec<Principal>,
}

#[derive(CandidType, Deserialize, Clone)]
struct UserProfile {
    user_principal: Principal,
    username: String,
    avatar_url: String,
    bio: String,
}

thread_local! {
    static POSTS: RefCell<HashMap<u64, Post>> = RefCell::new(HashMap::new());
    static NEXT_ID: RefCell<u64> = RefCell::new(0);
    static USERS: RefCell<HashMap<Principal, UserProfile>> = RefCell::new(HashMap::new());
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

#[update]
fn register_user(username: String, avatar_url: String, bio: String) {
    let principal = ic_cdk::caller();
    let final_avatar = if avatar_url.trim().is_empty() {
        DEFAULT_AVATAR_URL.to_string()
    } else {
        avatar_url
    };

    let profile = UserProfile {
        user_principal: principal,
        username,
        avatar_url: final_avatar,
        bio,
    };

    USERS.with(|users| {
        users.borrow_mut().insert(principal, profile);
    });
}

#[query]
fn get_my_profile() -> Option<UserProfile> {
    let principal = ic_cdk::caller();
    USERS.with(|users| users.borrow().get(&principal).cloned())
}

#[update]
fn update_profile(username: String, avatar_url: String, bio: String) {
    let principal = ic_cdk::caller();

    USERS.with(|users| {
        let mut users = users.borrow_mut();
        if let Some(user) = users.get_mut(&principal) {
            user.username = username;
            user.avatar_url = if avatar_url.trim().is_empty() {
                DEFAULT_AVATAR_URL.to_string()
            } else {
                avatar_url
            };
            user.bio = bio;
        }
    });
}