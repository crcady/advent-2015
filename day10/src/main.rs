fn main() {
    let mut seq: Vec<u8> = vec![1, 3, 2, 1, 1, 3, 1, 1, 1, 2];

    for _ in 0..50 {
        let mut new_seq: Vec<u8> = Vec::new();
        let mut count = 0u8;
        let mut last = 0u8;

        for current in seq {
            if count == 0 {
                last = current;
                count = 1;
            } else {
                if last == current {
                    count += 1;
                } else {
                    new_seq.push(count);
                    new_seq.push(last);
                    count = 1;
                    last = current;
                }
            }
        }
        new_seq.push(count);
        new_seq.push(last);

        seq = new_seq;
    }

    println!("After 40 reps, we have {} characters", seq.len());
}
