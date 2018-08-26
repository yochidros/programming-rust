
mod queue;

fn main() {
    let mut q = queue::Queue::new();

    q.push('a');
    q.push('b');
    q.push('3');

    assert_eq!(q.pop(), Some('a'));
    q.push('x');

    let (mut older, mut younger) = q.split();
    // q is moved (older, younder)

    older.push('s');
    younger.push('h');

    assert_eq!(older.len(), 3);
    assert_eq!(younger.len(), 2);
}
