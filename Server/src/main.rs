use std::net::UdpSocket;
use std::{thread, time};
use std::env;

fn get_milis(dur : std::time::Duration) -> u64 {
    let nanos = dur.subsec_nanos() as u64;
    let ms = (1000*1000*1000 * dur.as_secs() + nanos)/(1000 * 1000);
    ms
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("small args");
        std::process::exit(0x0100);
    }

    let bind = &args[1];
    let remoteAdr = &args[2];
    let sizeToTest: u8 = args[3].parse::<u8>().unwrap();
   
    let socket = UdpSocket::bind(bind).expect("couldn't bind to address");
    println!("binded to {:?} ", bind);

    socket.connect(remoteAdr);
    println!("connected to {:?} ", remoteAdr);

    socket.send(&[sizeToTest]).expect("couldn't send data");            
    println!("Size sent with {:?} " , sizeToTest);      

    let mut buf = vec![1; sizeToTest as usize];

    let mut rbuf = vec![0; sizeToTest as usize];

    loop {
         println!("----------------");
        thread::sleep(time::Duration::new(1, 0));

        let beforesend = time::Instant::now();
        socket.send(&mut buf).expect("couldn't send data");
        
        socket.recv(&mut rbuf);
        println!("the latency was {:?}", get_milis(time::Instant::now().duration_since(beforesend)));
    }
   
}