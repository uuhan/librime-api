use librime_api::*;

fn main() {
    let mut rime = RimeBuilder::new()
        .shared_data_dir("./rime-ice")
        .user_data_dir("./rime-user")
        .distribution_name("RIME")
        .distribution_code_name("RIME")
        .distribution_version("RIME")
        .app_name("rime-query")
        .log_dir("./rime-log")
        .build()
        .expect("create rime failed");

    rime.set_notification_handler(|id, ty, msg| {
        println!("[{}] type: {}, msg: {}", id, ty, msg);
    });

    let session = rime.create_session();

    for key in "china man".chars() {
        session.process_char(key);

        let ctx = session.context();
        println!("preedit: {}", ctx.preedit());
        println!("total candidates: {}", ctx.num_candidates());

        for i in ctx.all_candidates() {
            println!("cans: {}", i);
        }
    }

    std::thread::park();
}
