use crate::dbdiesel::model::*;
use crate::dbdiesel::schema::users;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use diesel::prelude::*;



pub async fn create_user(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(users::table)
                .values(new_user)
                .returning(User::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

// pub async fn update_user(
//     State(pool): State<deadpool_diesel::postgres::Pool>,
//     Json(update_user): Json<User>,
// ) -> Result<String, (StatusCode, String)> {
//     let conn = pool.get().await.map_err(internal_error)?;
//     // let mut up_fields = Vec::new();
    
//     // if !update_user.name.is_empty() {
//     //     up_fields.push(name.eq(update_user.name));
//     // }

//     let _ = conn
//         .interact(|conn| {
//             diesel::update(users::table)
//                 // .filter(users::id.eq(id))
//                 .set((name.eq(update_user.name)))
//                 // .returning(User::as_returning())
//                 // .get_result(conn)
//                 .execute(conn)
//         })
//         .await
//         .map_err(internal_error)?
//         .map_err(internal_error)?;
//     Ok(String::from("Update success!"))
// }

pub async fn list_users(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| users::table.select(User::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn delete_users(
    Path(ids): Path<String>,
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<String, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let mut id_vec: Vec<i32> = Vec::new();
    let items: Vec<&str> = ids.split(",").collect();
    for obj in items {
        let _ = &id_vec.push(obj.parse().unwrap());
    }
    let _ = conn
        // .interact(|conn| users::table.select(User::as_select()).load(conn))
        .interact(|conn| {
            diesel::delete(users::table.filter(users::id.eq_any(id_vec))).execute(conn)
        })
        .await
        .map_err(internal_error)?;
    Ok(String::from("删除成功！"))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
