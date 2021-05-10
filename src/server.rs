use actix_web::{App,get,post,HttpResponse,HttpServer};
use std::thread;
use std::boxed::Box;
use std::path::Path;
use crossbeam_channel::*;

enum ServerCommand {
    Quit
}

pub struct Server {
    server_thread: Option<thread::JoinHandle<()>>,
    sender: Sender<ServerCommand>
}

impl Server {
    pub fn new(_port: u16) -> Self {
        let (sender,receiver) = bounded::<ServerCommand>(4);
        Self {  
            server_thread: Some(thread::spawn(move || {
                let _server = HttpServer::new(|| App::new(

                    )
                );
            })),
            sender: sender
        }
    }
    pub fn add_file(&self, id: u32, path: &Path) {

    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let server_thread = self.server_thread.take().unwrap();
        server_thread.join().unwrap();
    }
}