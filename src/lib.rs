extern crate zmq;

#[no_mangle]
pub extern fn zmq_has_test() {
    println!("zmq has ipc: {:?}", zmq::has("ipc"));
    println!("zmq has curve: {:?}", zmq::has("curve"));
    println!("zmq has unknown: {:?}", zmq::has("unknown"));
}

#[cfg(test)]
mod tests {
   use super::*;

    #[test]
    fn it_works() {
        zmq_has_test();
    }
}


