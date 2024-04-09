

struct Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p.len()==0 {
            return p==s;
        }
        let p=p.into_bytes();
        let s=s.into_bytes();

        let mut ip=0;// index for p
        let mut is=0;// index for s
        while ip<p.len() && is<s.len() {
            let pat=p[ip];
            let ch=s[is];
            is+=1;

            if pat!=ch && pat!=b'?' && pat!=b'*' {
                return false;
            }

            if pat==b'*' && p.get(ip+1)!=s.get(is) {
                continue;
            }
            ip+=1;
        }

        is==s.len()
    }
}



fn main() {
    let tests=[("aa","a"),("aa","*"),("cb","?a"),("acdcb","a*c?b")];
    for (s,p) in tests {
        let matched=Solution::is_match(s.to_owned(),p.to_owned());
        println!("{matched}");
    }
}
