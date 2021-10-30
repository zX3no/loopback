mod loopback;
mod windefs;

use loopback::Loopback;
fn main() {
    //loopback <pid> <filename>
    // let args: Vec<_> = std::env::args().skip(1).collect();

    // if args.len() != 2 {
    //     return;
    // }

    // let pid = args[0].parse::<i32>().unwrap_or(0);

    // if pid == 0 {
    //     return;
    // }

    // let file_name = &args[1];

    let loopback = Loopback::new();

    loopback.run(7744, &String::from("test.wav"));
    // loopback.run(pid, file_name);
}
