pub fn run<'a, I>(args: I)
where
    I: Iterator<Item = &'a str>,
{
    let output: String = args.collect::<Vec<&str>>().join(" ");
    println!("{}", output);
}