use super::Solution;
// restaurants[i] = [id, rating, veganFriendly, price, distance]

impl Solution {
    pub fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        let mut tmp = restaurants
            .iter()
            .filter(|&item| {
                item[2] >= vegan_friendly && item[3] <= max_price && item[4] <= max_distance
            })
            .map(|item| (item[0], item[1]))
            .collect::<Vec<(i32, i32)>>();

        tmp.sort_by(|(left_id, left_rating), (right_id, right_rating)| {
            if left_rating != right_rating {
                return right_rating.cmp(left_rating);
            }
            return right_id.cmp(left_id);
        });

        tmp.iter().map(|item| item.0).collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn min_space_wasted_k_resizing_should_work() {
        assert_eq!(
            vec![3, 1, 5],
            Solution::filter_restaurants(
                vec![
                    vec![1, 4, 1, 40, 10],
                    vec![2, 8, 0, 50, 5],
                    vec![3, 8, 1, 30, 4],
                    vec![4, 10, 0, 10, 3],
                    vec![5, 1, 1, 15, 1]
                ],
                1,
                50,
                10
            )
        );

        assert_eq!(
            vec![4, 5],
            Solution::filter_restaurants(
                vec![
                    vec![1, 4, 1, 40, 10],
                    vec![2, 8, 0, 50, 5],
                    vec![3, 8, 1, 30, 4],
                    vec![4, 10, 0, 10, 3],
                    vec![5, 1, 1, 15, 1]
                ],
                0,
                30,
                3
            )
        );
    }
}
