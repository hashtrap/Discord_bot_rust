use strsim::normalized_damerau_levenshtein;


fn sim_exp(target:&[String],user:&[String])->f64
{
    let mut score:f64=0.0;
    let mut index=0;
    while target.len()==user.len() && index<target.len()
        {
            score+=normalized_damerau_levenshtein(&user[index],&target[index]);
            index+=1;
        }

    score=score/target.len() as f64;

     score * 100.0
}

fn input_clean(target:&str)->Vec<String>
{
    

    target.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}



pub fn run_similarity(user:&str,target:&String)->f64
{
    let clean_user=input_clean(user);

    let target=target.split(" ").map(| line | line.to_string()).collect::<Vec<String>>();

    let score=sim_exp(&clean_user,&target);

    score
}


#[cfg(test)]

mod tests
{
    use super::*;


    #[test]
    fn test_clean()
    {
        let a = String::from("   as **(**((( aaass");

        assert_eq!(input_clean(&a), ["as","aaass"]);
    }


    #[test]
    fn sim_check()
    {
        let user=String::from("k bb bb ");
        let target=String::from("aaaa bb bb");
        let score=run_similarity(&user,&target);

        println!("My method score: {}",score)
    }
}