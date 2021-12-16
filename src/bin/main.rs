fn main()  -> std::io::Result<()> {
    println!("API start!!!!");
    jaist_query_api::controller::server_run()
}
