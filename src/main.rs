
mod my_info;
mod array;


fn main()
{
    my_info::print_name();
    my_info::details::print_age(44);
    my_info::details::birth_date::birth_date();

    array::print_array();

}