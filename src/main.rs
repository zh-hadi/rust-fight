
// mod my_info;
// mod array;
// mod rust_struct;
// mod text_format;
// mod condition_oparator;
// mod hash_map;
// mod enumaration;
// mod user_io;
// mod json_data;
// mod use_tokio;
// mod learn_time;
// mod learn_axum;
mod sqlx_learn;


#[tokio::main]
async fn main()
{
    // my_info::print_name();
    // my_info::details::print_age(44);
    // my_info::details::birth_date::birth_date();

    // array::print_array();
    // rust_struct::learn_struct();
    // text_format::print_text();
    // condition_oparator::load();
    // hash_map::hash_map_load();
    // enumaration::enumaration_load();
    // user_io::user_io_load();
    // json_data::json_convert::initialization();
    // use_tokio::learn_async::initialization().await;
    // learn_time::initialization();

    //---- learn async tokio more ----
    // let handle = tokio::spawn(async {
    //     learn_axum::start::initialization().await;
    // });
    // async_fn_check();

    
    // println!("Main() end");
    // handle.await.unwrap();
    // --- end async tokio -----

    sqlx_learn::initialization().await;
}

// for asyn test
// pub fn async_fn_check()
// {
//     println!("sync function");
// }