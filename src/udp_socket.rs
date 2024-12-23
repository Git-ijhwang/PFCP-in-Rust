use std::net::UdpSocket;
use log::{debug, error, info, trace, warn};

pub fn socket_create (src_bind: String) -> Result<UdpSocket, String> {
    let socket = UdpSocket::bind(&src_bind).map_err(|e| {
        format!("UDP Socket binding Error on {}: {}", src_bind, e)
    })?;

    socket.set_nonblocking(false).map_err(|e| {
        format!("Failed to set non-blocking mode: {}", e)
    })?;

    Ok(socket)
}

pub fn send_udp_data(data: &[u8], ip: &str, port: u16)
-> Result<usize, String> {
    // Create a new UDP socket
    // Binds to an available port on the local machine
    let socket;
    let ret = UdpSocket::bind("0.0.0.0:0");
    match ret {
        Ok(v) => socket = v,
        _ => return Err ("Socket Open Error occoured".to_string()),
    }

    // Format the target address
    let addr = format!("{}:{}", ip, port);

    // Send the data
    let ret = socket.send_to(data, addr);
    match ret {
        Ok(v) => Ok(v),
        _ => Err ("Send Error occoured".to_string()),
    }
}
