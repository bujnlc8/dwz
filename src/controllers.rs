use crate::models::*;

use super::models::CommonError;
use crate::establish_connection;
use crate::schema::dwz;
use crate::schema::dwz::dsl::*;
use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use fastmurmur3;

pub fn get_redirect_url(url: &str) -> String {
    let connection = &mut establish_connection();
    let record = dwz
        .filter(short_url.eq(url))
        .filter(valid_time.gt(Local::now().naive_local()))
        .first::<Dwz>(connection);
    match record {
        Ok(e) => e.redirect_url,
        Err(_e) => "".to_string(),
    }
}

pub fn get_record(url: &str) -> QueryResult<Dwz> {
    let connection = &mut establish_connection();
    dwz.filter(short_url.eq(url)).first::<Dwz>(connection)
}

pub fn insert_data(url: &str, valid: &str) -> Result<String, CommonError> {
    let url = &url.trim().to_string();
    if url.is_empty() {
        return Err(CommonError {
            message: "url is blank!".to_string(),
        });
    }
    let parsed_valid_time;
    match NaiveDateTime::parse_from_str(valid, "%Y-%m-%d %H:%M:%S") {
        Ok(e) => parsed_valid_time = e,
        Err(e) => {
            return Err(CommonError {
                message: format!("parse datetime error: {}", e),
            })
        }
    }
    let u = fastmurmur3::hash(url.as_bytes());
    let mut h = format!("{:x}", u)[..6].to_string();
    let record = get_record(&h);
    let mut exist: Option<Dwz> = None;
    match record {
        Ok(e) => exist = Some(e),
        Err(_e) => (),
    }
    // 如果存在，检查有效期
    let connection = &mut establish_connection();
    if let Some(..) = exist {
        let exist_ef = &exist.unwrap();
        if exist_ef.redirect_url == *url {
            if exist_ef.valid_time.eq(&Local::now().naive_local()) {
                return Ok(h);
            } else {
                // 更新有效期
                match diesel::update(dwz.filter(id.eq(exist_ef.id)))
                    .set(valid_time.eq(parsed_valid_time))
                    .execute(connection)
                {
                    Ok(_) => return Ok(h),
                    Err(e) => {
                        return Err(CommonError {
                            message: format!("update valid_time error: {}", e),
                        })
                    }
                }
            }
        } else {
            // 产生冲突
            h = format!("{:x}", u);
            h = h[h.len() - 6..].to_string();
        }
    }
    let new_dwz = NewDwz {
        short_url: h.as_str(),
        redirect_url: url,
        valid_time: NaiveDateTime::parse_from_str(valid, "%Y-%m-%d %H:%M:%S").unwrap(),
        create_time: Local::now().naive_local(),
    };
    match diesel::insert_into(dwz::table)
        .values(&new_dwz)
        .execute(connection)
    {
        Ok(_) => Ok(h),
        Err(e) => Err(CommonError {
            message: format!("insert data error: {}", e),
        }),
    }
}
