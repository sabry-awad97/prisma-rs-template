use prisma::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    log::info!("Starting application...");

    let client = PrismaClient::_builder().build().await.unwrap();

    let users = client
        .user()
        .find_many(vec![])
        .with(
            user::posts::fetch(vec![])
                .with(post::user::fetch().with(user::posts::fetch(vec![])))
                .skip(10)
                .take(5),
        )
        .exec()
        .await
        .unwrap();

    log::info!("Users: {:?}", users);

    let user = client
        .user()
        .create("user0".to_string(), "User 0".to_string(), vec![])
        .exec()
        .await
        .unwrap();
    log::info!("Created user: {:?}", user);

    let post = client
        .post()
        .create(
            "post0".to_string(),
            "Some post content".to_string(),
            user::id::equals(user.id.to_string()),
            vec![],
        )
        .exec()
        .await
        .unwrap();
    log::info!("Created post: {:?}", post);

    let post_with_user = client
        .post()
        .find_unique(post::id::equals("post0".to_string()))
        .with(post::user::fetch())
        .exec()
        .await
        .unwrap()
        .unwrap();
    log::info!("Post user: {:?}", post_with_user.user().unwrap());

    let user_with_posts = client
        .user()
        .find_unique(user::id::equals("user0".to_string()))
        .with(user::posts::fetch(vec![]))
        .exec()
        .await
        .unwrap()
        .unwrap();
    log::info!("User posts: {:?}", user_with_posts.posts().unwrap());

    let deleted_posts_count = client.post().delete_many(vec![]).exec().await.unwrap();
    log::info!("Deleted {} posts", deleted_posts_count);

    let deleted_users_count = client.user().delete_many(vec![]).exec().await.unwrap();
    log::info!("Deleted {} users", deleted_users_count);

    Ok(())
}
