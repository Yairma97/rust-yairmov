pub mod stream;
pub mod helloworld;

pub mod echo {
    include!("stream.rs");
}

pub mod hello {
    include!("helloworld.rs");
}