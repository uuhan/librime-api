macro_rules! rime_struct_init {
    ($type:ty) => {{
        let mut var: $type = unsafe { std::mem::zeroed() };
        var.data_size =
            (std::mem::size_of::<$type>() - std::mem::size_of_val(&var.data_size)) as i32;
        var
    }};
}
