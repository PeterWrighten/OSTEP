pub fn count_proc(input: &str) -> u32 {
    let words: Vec<&str> = input.split("||").collect();
    let counts: Vec<i32> = words.iter().map(|&word| word.split("&&").collect::<Vec<_>>().len() as i32 + 1).collect();
    let counts: Vec<i32> = counts
        .iter()
        .map(|&num|  (1..(num+1)).sum() )
        .collect();
    let ans: i32 = counts.iter().sum();
    ans as u32

}

#[cfg(test)]
mod tests {
    use super::count_proc;
    #[test]
    fn test_fork_count() {
        let ques = "fork() && fork() && fork() || fork() && fork() || fork() && fork()";
        let ans =  count_proc(ques);
        assert_eq!(ans, 22);
    }
}