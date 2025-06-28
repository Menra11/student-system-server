use crate::model::*;
use salvo::prelude::*;

#[handler]
pub async fn get_login(req: &mut Request) -> String { 
    let login_data: LoginData = req.parse_json().await.unwrap();
   format!("{:?}", login_data)
}