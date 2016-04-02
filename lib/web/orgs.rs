use iron::prelude::*;
use iron::status;
use org::OrgModel;
use rustc_serialize::json;
use router::Router;
use bodyparser;
use serde_json::Value;
use std::str::FromStr;

pub fn register_endpoints(mut r: Router) -> Router {
    r.get("/organizations", org_list);
    r.get("/organizations/:org_id", org_get);
    r.post("/organizations", org_create);
    r
}


#[derive(Debug, RustcEncodable)]
pub struct OrgResponse {
    id: i32,
    name: String,
    description: Option<String>,
}

impl From<OrgModel> for OrgResponse {
    fn from(o: OrgModel) -> OrgResponse {
        OrgResponse {
            id: o.id,
            name: o.name.clone(),
            description: o.description.clone(),
        }
    }
}

pub fn org_list(req: &mut Request) -> IronResult<Response> {
    let conn = get_pg_connection!(req);
    let results = itry!(OrgModel::list(&conn));
    let payload = results.into_iter().map(OrgResponse::from).collect::<Vec<_>>();
    let json = itry!(json::encode(&payload));
    Ok(Response::with((status::Ok, json)))
}

pub fn org_get(req: &mut Request) -> IronResult<Response> {
    let org_id = {
        let ref org_id_str = req.extensions
                                .get::<Router>()
                                .unwrap()
                                .find("org_id")
                                .unwrap_or("none");
        if let Ok(org_id) = i32::from_str(*org_id_str) {
            org_id
        } else {
            return Ok(Response::with(status::BadRequest));
        }
    };
    let conn = get_pg_connection!(req);
    let result = itry!(OrgModel::get(&conn, org_id));
    if let Some(model) = result {
        let json = itry!(json::encode(&OrgResponse::from(model)));
        Ok(Response::with((status::Ok, json)))
    } else {
        Ok(Response::with(status::NotFound))
    }

}

pub fn org_create(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Json>();
    match body {
        Ok(Some(body)) => {
            if let Some(obj) = body.as_object() {
                if let Some(name) = obj.get("name") {
                    let name = match *name {
                        Value::String(ref v) => v.clone(),
                        _ => return Ok(Response::with(status::BadRequest)),
                    };
                    let desc = match obj.get("description") {
                        Some(&Value::String(ref v)) => Some(v.clone()),
                        _ => None,
                    };
                    let o = OrgModel::new(name, desc);
                    let conn = get_pg_connection!(req);
                    itry!(o.create(&conn));
                    return Ok(Response::with(status::Created));
                }
            }
            Ok(Response::with(status::BadRequest))
        }
        Ok(None) => Ok(Response::with(status::BadRequest)),
        Err(err) => {
            error!("{}", err);
            Ok(Response::with(status::InternalServerError))
        }
    }

}
