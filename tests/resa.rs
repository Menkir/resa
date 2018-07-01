extern crate resa;
use resa::Solutions;
use resa::stackoverflow::*;

#[test]
#[ignore]
fn crate_test(){
    let mut s: StackOverflow = StackOverflow::search("Compiler error").unwrap();
    println!("{:?}", s.items.len());
    assert!(s.items.len() >= 1);
        
    s.filter(4);
    assert_eq!(s.items.len(), 4);

    for item in s.items{
        assert!(!item.link.is_empty());
    }

}