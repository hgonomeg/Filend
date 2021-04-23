use actix_web::{App,get,post,HttpResponse,HttpServer};
use std::any::Any;
use std::boxed::Box;

pub struct Server {
    server: Box<Any>
}

impl Server {
    pub fn new() -> Self {
        Self {  
            server: Box::from(
                HttpServer::new(|| App::new(

                    )
                )
            ) 
        }
    }
}