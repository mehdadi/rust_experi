use std::net::UdpSocket;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut size = [0; 1];
    let bind = &args[1];

    println!("listening to {:?}", bind);

    let socket = UdpSocket::bind(bind).expect("couldn't bind to address");
    let (number_of_bytes, src_addr) = socket.recv_from(&mut size).expect("Didn't receive data");
    
    let mut buf = vec![0; size[0] as usize];
    println!("size is {:?}", size);

    loop {
        let (number_of_bytes, src_2) = socket.recv_from(&mut buf).expect("Didn't receive data");
        
        if src_2 != src_addr {
            std::process::exit(0x0100);
        }
               
        println!("packet came from {:?}", src_2);

        let filled_buf = &mut buf[..number_of_bytes];
        
        println!("sending back the read of : {:?}", number_of_bytes);

        socket.send_to(&[number_of_bytes as u8], src_2).expect("couldn't send data"); 
    }
}