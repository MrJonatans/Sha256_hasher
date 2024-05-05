
pub fn hash_finder<'a>(args:(i32,i32)) ->Vec<(i32,&'a str)>{

    let mut result:Vec<(i32,&str)> = Vec::new();
    let (num_zeros, iter) = args;
    let val:i32 = 1;

    while result.len().into() != iter{
        generate_hash(val);
    }

    return result
}