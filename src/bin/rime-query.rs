use librime_api::prelude::*;

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

    {
        // let module = RimeModule::find("core");
        // println!("find module: {}", util::safe_text((*module).module_name));
        // let module = RimeModule::find("mymodule");
        // println!("find module: {:?}", module);

        // let module = RimeModule::find("mymodule");
        // println!("find module: {}", util::safe_text((*module).module_name));
    }

    session.process_string("math");
    let ctx = session.context();

    println!("preedit: {}", ctx.preedit());
    println!("total candidates: {}", ctx.num_candidates());

    for (k, v) in ctx.all_candidates() {
        println!("cans: {}, {}", k, v);
    }

    // for key in "man".chars() {
    //     session.process_char(key);
    //
    //     let ctx = session.context();
    //     println!("preedit: {}", ctx.preedit());
    //     println!("total candidates: {}", ctx.num_candidates());
    //
    //     for (k, v) in ctx.all_candidates() {
    //         println!("cans: {}, {}", k, v);
    //     }
    // }

    Rime::Destory();
}
