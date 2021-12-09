fn main()  -> std::io::Result<()> {
    println!("Hello, world!");
    jaist_query_api::controller::server_run()
}
