use super::transfer;

pub fn parse_args(args: &[String]) -> Result<transfer::TransferRequest, String> {
    let mut res = transfer::TransferRequest{sources: vec![], destination: ""};
    let arg_count = args.len();
    if arg_count < 2 {
        return Err(format!("Usage: {} SRC [SRC ...] DEST", args[0]))
    }
    for i in 1..arg_count-1 {
        res.sources.push(&args[i]);
    }
    res.destination = &args[arg_count-1];
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_args_into_sources_and_dest_one_file() {
        let args = vec![String::from("rucp"), String::from("src.txt"), String::from("dest.txt")];
        let transfer_request = parse_args(&args).expect("invalid input");
        assert_eq!(transfer_request.sources, vec!["src.txt"]);
        assert_eq!(transfer_request.destination, "dest.txt");
    }

    #[test]
    fn parse_args_into_sources_and_dest_serveral_files() {
        let args = vec![
            String::from("rucp"),
            String::from("one.txt"),
            String::from("two.txt"),
            String::from("dest")
        ];
        let transfer_request = parse_args(&args).expect("invalid input");
        assert_eq!(transfer_request.sources, vec!["one.txt", "two.txt"]);
        assert_eq!(transfer_request.destination, "dest");
    }
}
