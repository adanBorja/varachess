use gtest::{Log, Program, System};
use varachess_io::ChessMessageIn;

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    program.send(2, String::from("INIT MESSAGE"));
    program.send(2, ChessMessageIn::RequestStartGame);
}
