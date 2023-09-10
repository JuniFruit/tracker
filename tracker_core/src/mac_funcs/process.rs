use std::io::Read;

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
}

impl ProcessInfo {
    pub fn new(name: &str, pid: u32) -> Self {
        Self {
            name: name.to_string(),
            pid,
        }
    }
}

pub fn child_stream_to_vec<R>(mut stream: R) -> Vec<u8>
where
    R: Read + Send + 'static,
{
    let out: Vec<u8> = vec![];
    let mut vec = out.clone();
    loop {
        let mut buf = [0];
        match stream.read(&mut buf) {
            Err(err) => {
                println!("{}] Error reading from stream: {}", line!(), err);
                break;
            }
            Ok(got) => {
                if got == 0 {
                    break;
                } else if got == 1 {
                    vec.push(buf[0])
                } else {
                    println!("{}] Unexpected number of bytes: {}", line!(), got);
                    break;
                }
            }
        }
    }
    out
}
