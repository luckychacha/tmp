pub mod lt_00000_best_line;
pub mod lt_00000_jianzhi_is_symmetric;
pub mod lt_00067_jz_find_maximum_xor;
pub mod lt_00131_partition;
pub mod lt_00191_hamming_weight;
pub mod lt_00214_shortest_palindrome;
pub mod lt_00227_calculate;
pub mod lt_00274_h_index;
pub mod lt_00394_decode_string;
pub mod lt_00438_find_anagrams;
pub mod lt_00445_add_two_numbers;
pub mod lt_00647_count_substrings;
pub mod lt_00665_check_possibility;
pub mod lt_00682_cal_points;
pub mod lt_00767_reorganize_string;
pub mod lt_00775_is_ideal_permutation;
pub mod lt_00823_num_factored_binary_trees;
pub mod lt_00883_projection_area;
pub mod lt_00915_partition_disjoint;
pub mod lt_01171_remove_zero_sum_sublists;
pub mod lt_01177_can_make_pali_queries;
pub mod lt_01252_odd_cells;
pub mod lt_01315_sum_even_grandparent;
pub mod lt_01333_filter_restaurants;
pub mod lt_01415_get_happy_string;
pub mod lt_01528_restore_string;
pub mod lt_01540_can_convert_string;
pub mod lt_01669_merge_in_between;
pub mod lt_01678_interpret;
pub mod lt_01702_maximum_binary_string;
pub mod lt_01705_eaten_apples;
pub mod lt_01827_min_operations;
pub mod lt_01959_min_space_wasted_k_resizing;
pub mod lt_02075_decode_ciphertext;
pub mod lt_02134_min_swaps;
pub mod lt_02216_min_deletion;
pub mod lt_02279_maximum_bags;
pub mod lt_02300_successful_pairs;

pub struct Solution();

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
//
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
