extern crate rpc_client;

fn main() {

    let url = "someurl".to_string();
    
    match rpc_client::request::call(url) {
        Ok(res) => println!("{}", res),
        Err(err) => println!("{}", err)
    }
}
