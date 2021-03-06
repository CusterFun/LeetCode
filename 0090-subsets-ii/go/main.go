package main

var result [][]int

func subsetsWithDup(nums []int) [][]int {
	result = make([][]int, 0)
	hm := make(map[int]int, 0)
	for i := 0; i < len(nums); i++ {
		count := 1
		if _, ok := hm[nums[i]]; ok {
			count += hm[nums[i]]
		}
		hm[nums[i]] = count
	}
	n := len(hm)
	uniqueNums := make([]int, n)
	counts := make([]int, n)
	k := 0
	for i := 0; i < len(nums); i++ {
		if _, ok := hm[nums[i]]; ok {
			uniqueNums[k] = nums[i]
			counts[k] = hm[nums[i]]
			k++
			delete(hm, nums[i])
		}
	}
	backtrack(uniqueNums, counts, 0, []int{})
	return result
}
func backtrack(uniqueNums, counts []int, k int, path []int) {
	if k == len(uniqueNums) {
		snapshot := make([]int, len(path))
		copy(snapshot, path)
		result = append(result, snapshot)
		return
	}
	for count := 0; count <= counts[k]; count++ {
		for i := 0; i < count; i++ {
			path = append(path, uniqueNums[k])
		}
		backtrack(uniqueNums, counts, k+1, path)
		for i := 0; i < count; i++ {
			path = path[:len(path)-1]
		}
	}
}
