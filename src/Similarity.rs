

use strsim::normalized_levenshtein;


fn calculate_lyric_similarity_with_typos(user_input: &[String], target_lyrics: &[String]) -> f64{
    let u_len = user_input.len();
    let t_len = target_lyrics.len();

    if u_len == 0 && t_len == 0 { return 100.0; }
    if u_len == 0 || t_len == 0 { return 0.0; }

    // DP matrix storing f64 for fractional typo costs
    // NOTE: Fixed your loops to go up to <= u_len and <= t_len to properly initialize the matrix boundary!
    let mut dp = vec![vec![0.0; t_len + 1]; u_len + 1];

    for i in 0..=u_len {
        dp[i][0] = i as f64;
    }
    for j in 0..=t_len {
        dp[0][j] = j as f64;
    }

    for i in 1..=u_len {
        for j in 1..=t_len {
            let u_word = user_input[i - 1].to_string();
            let t_word = target_lyrics[j - 1].to_string();

            // FIX: If one word is massively longer than the other, it's NOT a typo.
            // It's a completely different/malformed token block. Cap its maximum forgiveness.
            let u_word_len = u_word.len() as f64;
            let t_word_len = t_word.len() as f64;
            let len_ratio = u_word_len.min(t_word_len) / u_word_len.max(t_word_len);

            let typo_cost = if len_ratio < 0.3 {
                // If one word is more than 3x longer than the other, give it 0 typo forgiveness (full penalty)
                1.0
            } else {
                // Otherwise, calculate character similarity normally
                1.0 - normalized_levenshtein(&u_word, &t_word)
            };

            let substitution_cost = dp[i - 1][j - 1] + typo_cost;
            let deletion_cost = dp[i - 1][j] + 1.0;
            let insertion_cost = dp[i][j - 1] + 1.0;

            dp[i][j] = substitution_cost.min(deletion_cost).min(insertion_cost);
        }
    }

    let total_edits = dp[u_len][t_len];
    let max_possible_words = u_len.max(t_len) as f64;

    ((max_possible_words - total_edits) / max_possible_words) * 100.0
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

pub fn run_similarity(user:&str,target:&[String])->f64
{
    let clean_user=input_clean(user);

    let score=calculate_lyric_similarity_with_typos(&clean_user,target);

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
        let user=String::from("asdfADFASSDFGSDGHDFGHDFGHDFGHDFGHDFASDFasdf bb bb ");
        let target=vec![String::from("aaaa"),String::from("bb"),String::from("bb")];

        let score=run_similarity(&user,&target);

        println!("{}", score);
    }
}