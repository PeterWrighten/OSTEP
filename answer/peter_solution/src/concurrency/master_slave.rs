use std::thread;
use std::thread::JoinHandle;
use std::sync::Arc;

fn check_num(num: &i32) -> bool {
    if *num < 2 {
        return false;
    } 

    for i in 0..(*num as f64).sqrt() as i32 {
        if num % i == 0 {
            return false;
        }
    }

    true
}

fn check_tot(l: &i32, r: &i32,  tot: &mut Arc<Vec<i32>>) {
    for i in *l..*r {
        if check_num(&i) {
            *tot.push(i);
        }
    }
}

#[test]
fn test_ms() {
    const SUM: i32 = 999999;
    let mut threads: Arc<Vec<JoinHandle<()>>> = Arc::new(Vec::new());
    let mut nums: Arc<Vec<i32>> = Arc::new(Vec::new());
    let (mut mi, mut mx): (Arc<i32>, Arc<i32>) = (Arc::new(0), Arc::new(0));
    for i in 1..4 {
        *mx = *mi + i * (SUM / 4);
        if *mx > SUM {
            *mx = SUM;
        }
        let thread = thread::spawn(|| {
            check_tot(&mi, &mx, &mut nums);
        });
        threads.push(thread);
        mi = mx;
    }

    for i in 0..4 {
        threads[i].join();
    }

    let tot = nums.iter().fold(0, |acc, num| acc + num);

    assert_eq!(tot, 78498);
}