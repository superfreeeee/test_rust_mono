fn main() {
    println!("Hello, world!");
}

// Vec<Option<u8>> 转 Option<Vec<u8>>
#[test]
fn test_collect_option() {
    let mut count = 0;
    let vec = vec![Some(1u8), Some(2u8), None, Some(3u8)]
        .into_iter()
        .map(|num| {
            count += 1;
            num
        })
        .collect::<Option<Vec<_>>>();
    // 1. meet None => None, and early return
    assert_eq!(vec, None);
    assert_eq!(count, 3);

    let vec = vec![Some(1u8), Some(2u8), Some(4u8), Some(3u8)]
        .into_iter()
        .collect::<Option<Vec<_>>>();
    // 2. all Some => Vec<u8>
    assert_eq!(vec, Some(vec![1, 2, 4, 3]));

    let vec = vec![Some(1u8), Some(2u8), None, Some(3u8)]
        .into_iter()
        .collect::<Vec<_>>();
    // 3. normal Vec
    assert_eq!(vec, vec![Some(1u8), Some(2u8), None, Some(3u8)]);
}
