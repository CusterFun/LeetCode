package main

var result [][]int

func subsets(nums []int) [][]int {
	result = make([][]int, 0)
	backtrack(nums, 0, []int{})
	return result
}
func backtrack(nums []int, k int, path []int) {
	if k == len(nums) {
		snapshot := make([]int, len(path))
		copy(snapshot, path)
		result = append(result, snapshot)
		return
	}
	backtrack(nums, k+1, path)
	path = append(path, nums[k])
	backtrack(nums, k+1, path)
	path = path[:len(path)-1]
}
