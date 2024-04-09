use http::request::Request;
use server::Server;

fn main () {
  // let get_request = Method::GET;
  // let delete_request = Method::DELETE;
  // let post_request = Method::POST;
  // let put_request = Method::PUT;

  let server = Server::new("127.0.0.1:8080".to_string());
  server.run();
}

//iplementation block that would holds the logic(functionality we need for the struct)
//(method and associated functions are functions that are asoccaited with the struct type) double colons syntax are used to access to asocaited functions . the new keyword is used inside an impl block. Associated functions are associated with a type rather than an instance of the type, and they are often used to create instances of the type.


//Explaining fn new and associated function like a five year old..  Alright, let's imagine you have a magic box, and this magic box can create special toys. Now, this box is so special that it doesn't belong to any particular toy; it's just a magical toy-making box. In Rust, we call this special box an "associated function." It's like a helper function that's associated with a type (like a toy-making box is associated with toys), but it doesn't depend on a specific toy you already have – it helps you make new toys of that type. So, when you want a new toy, you can ask the magic box (associated function) to create one for you. In your Rust code, the fn new() part is like asking the magic box to make a new toy (in your case, a new Server). The magic box knows how to create toys (instances of the type) based on what you tell it (the parameters you give, like the address in your Server::new("127.0.0.1:8080".to_string()) example).

// So, in simple terms, an associated function is like a magic box that helps you make new things of a certain kind, but it's not tied to any specific thing you already have – it's just a special helper for creating more of those things.

// sruct blocks that holds data
#[allow(dead_code)]
mod server {
  pub struct Server {
    addr: String,
    }
    impl Server {
      pub fn new(addr: String) -> Self {
        Self { 
          addr
        }
      }
      pub fn run(self) {
        println!("Server is listening on {}",  self.addr);
      }
    }
}
// it is accepted that the main constructor for a struct is called New. we can as well call that anything and the compiler would not complain

// method accept a special first parameter/arguement called self.

//In every struct . there is a key word called the uppercase Self which is alais of the Struct name.

// use modules to organise code and even put more modules inside other modules

mod http {
    pub mod request {
      use super::method::Method;
     pub struct Request {
      path: String,
      query_string: Option<String>,
      method: Method,
    }
}

 pub mod method {
    pub enum Method {
        GET,
        PUT,
        DELETE,
        POST,
        CONNECT,
        HEAD,
        OPTIONS,
        TRACE,
        PATCH
    }
  }
}

// every file in rust is treated as a module
// in memory enums are represented by numbers
// moving each struct into a module