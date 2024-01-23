/// # Description
/// This function will solve the order two knapsack problem, meaning it will return a subset of the given list so that the sum of all items in in the range of \[k/2, k\].
/// If such a solution exists it will return `Some(Vec<f64)`. No no solution is possible `None` wil be returned 
///
/// # Parameters
///  * list: `&Vec<f64>` - This represents the input list which is a vector of f64s. These must be positive or the algorithm in not guaranteed to work.
///  * target: `f64` - This represents the target which defines the range of solutions. This must also be positive numbers in order for the algorithm to work properly.
pub fn order_two_knapsack(list: &Vec<f64>, target: f64) -> Option<Vec<f64>> {
    if let Some(num) = list
        .iter()
        .filter(|num| **num >= target / 2.0 && **num <= target)
        .nth(0)
    {
        return Some(vec![*num]);
    }

    let mut sum: f64 = 0.0;
    let subset: Vec<f64> = list
        .iter()
        .filter(|num| **num < target / 2.0)
        .take_while(|num| {
            let ret = sum < target / 2.0;
            sum += *num;
            ret
        })
        .map(|num| *num)
        .collect();

    if subset.iter().sum::<f64>() < target / 2.0 {
        return None;
    }

    Some(subset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_solution_returns_none() {
        let list: Vec<f64> = vec![1.1, 2.2, 3.3];
        assert!(order_two_knapsack(&list, 20.0).is_none())
    }

    #[test]
    fn single_answer_solution() {
        let list: Vec<f64> = vec![1.1, 2.2, 14.5];
        let ans = order_two_knapsack(&list, 20.0);
        assert!(ans.is_some());

        let ans = ans.unwrap();
        assert_eq!(ans.len(), 1);
        assert_eq!(ans[0], 14.5);
    }
    #[test]
    fn multiple_answer_solution() {
        let list: Vec<f64> = vec![1.1, 1.2, 1.3, 1.4, 1.45];
        let ans = order_two_knapsack(&list, 3.0);
        assert!(ans.is_some());

        let ans = ans.unwrap();
        assert_eq!(ans.len(), 2);
        assert_eq!(ans[0], 1.1);
        assert_eq!(ans[1], 1.2);
    }

    #[test]
    fn multiple_answer_solution_with_large_nums() {
        let list: Vec<f64> = vec![10000202.0, 1.1, 302434.13, 1421.10, 1.2, 1.3, 1.4, 1.45];
        let ans = order_two_knapsack(&list, 3.0);
        assert!(ans.is_some());

        let ans = ans.unwrap();
        assert_eq!(ans.len(), 2);
        assert_eq!(ans[0], 1.1);
        assert_eq!(ans[1], 1.2);
    }
}