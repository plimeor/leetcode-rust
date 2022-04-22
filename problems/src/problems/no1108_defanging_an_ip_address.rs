// https://leetcode-cn.com/problems/defanging-an-ip-address/
struct Solution;

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace(".", "[.]")
    }
}


#[test]
fn test(){
}
