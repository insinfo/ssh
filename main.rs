extern crate ssh;
use std::io::{Read,Write};

use ssh::*;

fn main(){
    let mut session=Session::new().unwrap();
    session.set_host("pijul.org").unwrap();
    session.parse_config(None).unwrap();
    session.connect().unwrap();
    println!("{:?}",session.is_server_known());
    session.userauth_publickey_auto(None).unwrap();
    {
        let mut s=session.channel_new().unwrap();
        s.open_session().unwrap();
        s.request_exec(b"rm -Rf /tmp/testdir").unwrap();
        s.send_eof().unwrap();
        let mut buf=Vec::new();
        s.stdout().read_to_end(&mut buf).unwrap();
        println!("{:?}",std::str::from_utf8(&buf).unwrap());
    }
    {
        let mut scp=session.scp_new(RECURSIVE|WRITE,"/tmp").unwrap();
        scp.init().unwrap();
        scp.push_directory("testdir",0o755).unwrap();
        let buf=b"blabla\n".to_vec();
        scp.push_file("test file",buf.len(),0o644).unwrap();
        scp.write(&buf).unwrap();
    }
    /*
    {
        let mut scp=session.scp_new(WRITE,"/tmp").unwrap();
        scp.init().unwrap();
        let buf=b"blabla blibli\n".to_vec();
        scp.push_file("blublu",buf.len(),0o644).unwrap();
        scp.write(&buf).unwrap();
    }
    {
        let mut scp=session.scp_new(READ,"/tmp/blublu").unwrap();
        scp.init().unwrap();
        loop {
            match scp.pull_request().unwrap() {
                Request::NEWFILE=>{
                    let mut buf:Vec<u8>=vec!();
                    scp.accept_request().unwrap();
                    scp.reader().read_to_end(&mut buf).unwrap();
                    println!("{:?}",std::str::from_utf8(&buf).unwrap());
                    break;
                },
    Request::WARNING=>{
                    scp.deny_request().unwrap();
                    break;
                },
                _=>scp.deny_request().unwrap()
            }
        }
    }
     */
}
