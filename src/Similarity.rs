fn distance_calc()->f32
{
    let score:f32=0.0;


    score
}

fn order_check(lines_api:&Vec<String>,lines_user:String)->bool
{
    let user_tokens = input_clean(&lines_user);

    if lines_api.len() != user_tokens.len()
    {
         false
    }
    else {
        for index in 0..user_tokens.len()
        {
            if lines_api[index]!=user_tokens[index]
            {
                return false;
            }
        }

         true
    }
}

fn input_clean(target:&str)->Vec<String>
{
    let result=target.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    result
}


#[cfg(test)]

mod tests
{
    use super::*;


    #[test]
    fn test_clean()
    {
        let a=String::from("a");
        let b=String::from("     aasdfasdfasdf      ");
        let c=String::from("    @#$a a a & a   ");

        let a_fix=input_clean(&a);
        let b_fix=input_clean(&b);
        let c_fix=input_clean(&c);

        println!("Original:{:?}, the fix: {:?}",a,a_fix);
        println!("Original:{:?}, the fix: {:?}",b,b_fix);
        println!("Original:{:?}, the fix: {:?}",c,c_fix);
    }


    #[test]
    fn test_order_check()
    {
        let target=vec![String::from("a"),String::from("b"),String::from("c"),String::from("a"),String::from("a")];
        let goal=String::from("a b c a a");
        let fake_goal=String::from("a b c a abbs");


        assert_eq!(order_check(&target,goal),true);
        assert_eq!(order_check(&target,fake_goal),false);
    }
}