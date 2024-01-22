use librime_api::*;

fn main() {
    RimeBuilder::new()
        .shared_data_dir("./rime-ice")
        .user_data_dir("./rime-user")
        .distribution_name("RIME")
        .distribution_code_name("RIME")
        .distribution_version("RIME")
        .app_name("rime-query")
        .log_kind(RimeLogKind::StdErr)
        .build()
        .expect("create rime failed");

    Rime::SetNotificationHandler(|id, ty, msg| {
        println!("[{}] type: {}, msg: {}", id, ty, msg);
    });

    let session = Rime::CreateSession();

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
