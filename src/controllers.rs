use crate::models::*;

use crate::establish_connection;
use crate::schema::dwz;
use crate::schema::dwz::dsl::*;
use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use fastmurmur3;

pub fn get_redirect_url(url: &String) -> String {
    let connection = &mut establish_connection();
    let record = dwz
        .filter(short_url.eq(url))
        .filter(valid_time.gt(Local::now().naive_local()))
        .first::<Dwz>(connection);
    let record = match record {
        Ok(e) => e.redirect_url,
        Err(_e) => "".to_string(),
    };
    record
}

pub fn get_record(url: &String) -> QueryResult<Dwz> {
    let connection = &mut establish_connection();
    let record = dwz.filter(short_url.eq(url)).first::<Dwz>(connection);
    record
}

pub fn insert_data(url: &String, valid: &String) -> Result<String, diesel::result::Error> {
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
    if exist.is_some() {
        let exist_ef = &exist.unwrap();
        if exist_ef.redirect_url == *url {
            if exist_ef.valid_time.eq(&Local::now().naive_local()) {
                return Ok(h);
            } else {
                // 更新有效期
                diesel::update(dwz.filter(id.eq(exist_ef.id)))
                    .set(
                        valid_time
                            .eq(NaiveDateTime::parse_from_str(valid, "%Y-%m-%d %H:%M:%S").unwrap()),
                    )
                    .execute(connection)?;
                return Ok(h);
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
        Err(e) => Err(e),
    }
}
