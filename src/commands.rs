use clap::{Arg, ArgAction, Command};

// COMMANDS
pub fn get_command_domain() -> Command {
    let command_process: Command = Command::new("enoding")
        .short_flag('E')
        .long_flag("encoding")
        .about("Convert between encodings");
    return command_process;
}

// ARGS
pub fn get_args_domain() -> Vec<Arg> {
    let arg_out: Arg = Arg::new("out")
        .help("[path] Path to the out file")
        .short('o')
        .long("out")
        .action(ArgAction::Set);

    let arg_in: Arg = Arg::new("input")
        .help("[path] Path to the in file")
        .short('i')
        .long("input")
        .action(ArgAction::Set)
        .required(true);

    let arg_ending: Arg = Arg::new("lf")
        .help("[yes/no] Fix windows-style ending clrf to lf (unix-style line ending)")
        .short('l')
        .long("lf")
        .action(ArgAction::Set)
        .conflicts_with("lf");

    let arg_enc: Arg = Arg::new("encoding")
        .help("[utf-8/...] Desired encoding")
        .short('e')
        .long("encoding")
        .action(ArgAction::Set)
        .conflicts_with("encoding");

    // let arg_conv: Arg = Arg::new("convert")
    //     .help("[] Convert files with auto-detect (fix encoding)")
    //     .short('c')
    //     .long("convert")
    //     .action(ArgAction::Set);

    // let arg_detect: Arg = Arg::new("detect")
    //     .help("Detect encoding")
    //     .short('d')
    //     .long("detect")
    //     .action(ArgAction::Set)
    //     .conflicts_with("convert");

    return vec![arg_out, arg_in, arg_enc, arg_ending];
}
