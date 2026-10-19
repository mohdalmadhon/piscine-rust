pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len();
    let n = target.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

   
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            if &source[i - 1..i] == &target[j - 1..j] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + std::cmp::min(dp[i - 1][j - 1], std::cmp::min(dp[i][j - 1], dp[i - 1][j]));
            }
        }
    }

    dp[m][n]
}