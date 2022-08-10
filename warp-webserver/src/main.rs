#![deny(warnings)]

mod controller;
mod model;

use warp::Filter;
use crate::model::struct_model::{
    Store
};

use crate::controller::store_controller::{
    post_json,
    update_grocery_list,
    get_grocery_list,
    delete_json,
    delete_grocery_list_item
};


#[tokio::main]
async fn main() {

    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());
    
    let add_items = warp
        ::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);
    
    let get_items = warp
        ::get()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_grocery_list);
    
    let delete_item = warp
        ::delete()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(delete_json())
        .and(store_filter.clone())
        .and_then(delete_grocery_list_item);
    
    let update_item = warp
        ::put()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);
    
    let routes = add_items.or(get_items).or(delete_item).or(update_item);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}