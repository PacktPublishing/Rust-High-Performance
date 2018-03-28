extern crate env_logger;
#[macro_use]
extern crate log;

fn main() {
    env_logger::init();

    trace!("Logging {} small thing(s)", 1);
    debug!("Some debug information:  {}", "the answer is 42");
    info!("This is an interesting information");
    error!("An error happened, do something!");
}
