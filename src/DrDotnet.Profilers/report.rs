use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Local};

use std::path::{PathBuf, Path};
use std::io::BufWriter;
use std::fs::File;
use std::io::Write;

use crate::rust_protobuf_protos::interop::*;

// A Session refers to a profiling session. In conists in a process, a profiler, and a timestamp at which the profiling was done.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    session_id: Uuid,
    process_name: String,
    timestamp: DateTime<Local>,
    profiler_name: String,
}

impl Session {

    // Returns a Session from its UID and ProfilerData.
    // If the Session report is not present on the disk, it will be written at the same time.
    pub fn create_session_json(session_info: SessionInfo, profiler: ProfilerMetadata) {

        let process_name = std::env::current_exe().unwrap()
            .file_name().unwrap()
            .to_str().unwrap()
            .to_owned();

        let s = SessionInfo::new();

        // Serialize to JSON
        let json = protobuf_json_mapping::print_to_string(&s).unwrap();

        // Write session report
        let json_path = format!("{}/session.json", Session::get_directory(session_info.uuid));
        if !Path::exists(Path::new(&json_path)) {
            let mut session_stream = File::create(json_path).expect("Unable to create file");
            session_stream.write_all(json.as_bytes()).expect("Unable to write data");    
        };
    }

    // Create a new report for a given Session, ready to be filled up.
    pub fn create_report(&self, filename: String) -> Report {
        let path = PathBuf::from(format!(r"{}/{}", Session::get_directory(self.session_id), filename));
        let file = File::create(&path).unwrap();
        return Report { writer: BufWriter::new(file) };
    }

    pub fn get_root_directory() -> String {
        let directory_path = format!(r"{}/dr-dotnet", std::env::temp_dir().into_os_string().into_string().unwrap());
        std::fs::create_dir_all(&directory_path);
        return directory_path;
    }
    
    // Returns the directy path for this Session.
    pub fn get_directory(session_id: String) -> String {
        let directory_path = format!(r"{}/{}", Session::get_root_directory(), session_id.to_string());
        std::fs::create_dir_all(&directory_path);
        return directory_path;
    }
}

// A Session can contain several reports, which are usually files like markdown summaries or charts.
pub struct Report {
    pub writer: BufWriter<File>,
}

impl Report {
    pub fn write_line(&mut self, text: String) {
        self.writer.write(format!("{}\r\n", text).as_bytes()).unwrap();
    }

    pub fn new_line(&mut self) {
        self.writer.write(b"\r\n").unwrap();
    }
}