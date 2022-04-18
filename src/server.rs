use actix_web::{App,web,get,Result,HttpServer};
use actix_files::NamedFile;
use std::thread;
use std::collections::HashMap;
use std::path::{PathBuf,Path};
use std::sync::{Arc,Mutex};
use crossbeam_channel::*;
use crate::local_files::LocalFiles;

enum ServerCommand {
    Quit
}

pub struct Server {
    server_thread: Option<thread::JoinHandle<()>>,
    served_files: Arc<Mutex<HashMap<String,PathBuf>>>,
    sender: Sender<ServerCommand>
}

#[derive(Clone)]
struct ServerAppState {
    served_files: Arc<Mutex<HashMap<String,PathBuf>>>
}

#[get("/filend/{file_id}")] 
async fn index(file_id: web::Path<String>, data: web::Data<ServerAppState>) ->  Result<NamedFile> {
    let mut path = PathBuf::default();
    {
        path = data.served_files.lock().expect("Failed to access the mutex protecting served_files.")[&file_id.into_inner()].clone();
    }
    Ok(NamedFile::open(path)?)
}

#[actix_rt::main]
async fn server_main(state: ServerAppState, port: u16) -> std::result::Result<(), std::io::Error> {
    let mut state = Some(state);
    let server = HttpServer::new(move || {
        App::new()
            .data(state.clone().take().unwrap())
            .service(index)
    })
    .bind(format!("0.0.0.0:{}",port).as_str()).expect(format!("Could not bind to 0.0.0.0:{}",port).as_str())
    .shutdown_timeout(2)
    .run();
    eprintln!("Running!");
    server.await
}

impl Server {
    pub fn new(port: u16) -> Self {
        let (sender,receiver) = bounded::<ServerCommand>(4);
        let served_files = Arc::from(Mutex::from(HashMap::default()));
        let state = ServerAppState { served_files: served_files.clone() };
        let server_handle = Some(thread::spawn(move || {
            server_main(state, port).expect("Server crashed");
        }));

        Self {  
            server_thread: server_handle,
            sender: sender,
            served_files: served_files
        }
    }
    pub fn add_file(&self, id: u32, path: &Path) {
        self.served_files.lock().unwrap().insert(LocalFiles::id_to_hexstring(id),path.to_owned());
    }
    pub fn stop() {
        actix_rt::System::current().stop();
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        Server::stop();
        let server_thread = self.server_thread.take().unwrap();
        server_thread.join().expect("Could not join the server thread!");
    }
}