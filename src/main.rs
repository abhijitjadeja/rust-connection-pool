use std::vec::*;
struct Connection {
    is_open: bool,
}

impl Connection {
    fn execute(&mut self) {
        self.is_open = false;
    }
}

fn main() {
    let pool_contents = vec![
        Borrowable {
            object: Connection { is_open: true },
            id: 0,
        },
        Borrowable {
            object: Connection { is_open: true },
            id: 1,
        },
    ];
    let free_list = vec![0, 1];
    let inuse_list = vec![];
    let mut pool = Pool {
        pool: pool_contents,
        free_list,
        inuse_list,
    };

    pool.print_free_list();

    let b = pool.borrow();
    pool.print_free_list();
    let c = pool.borrow();
    pool.print_free_list();
    if let Some(mut borrowable) = b {
        let connection = &mut borrowable.object;
        println!("{:?}", connection.is_open);
        connection.execute();
        println!("{:?}", connection.is_open);
        pool.return_object(borrowable);
    }
    pool.print_free_list();
    pool.return_object(c.unwrap());
    pool.print_free_list();
}

fn remove_from_list(list: &mut Vec<i8>, item_to_remove: i8) {
    let mut i = 0;
    while i < list.len() {
        if list[i] == item_to_remove {
            list.remove(i);
        }
        i += 1;
    }
}

struct Pool<T> {
    pool: Vec<Borrowable<T>>,
    free_list: Vec<i8>,
    inuse_list: Vec<i8>,
}

struct Borrowable<T> {
    object: T,
    id: i8,
}

impl<T> Pool<T> {
    fn borrow(&mut self) -> Option<Borrowable<T>> {
        let free_list = &mut self.free_list;
        let pool = &mut self.pool;
        let b = pool.pop();
        if let Some(ref borrowable) = b {
        remove_from_list(free_list, borrowable.id);
        self.inuse_list.push(borrowable.id);
        }
        b
    }

    fn return_object(&mut self, borrowable: Borrowable<T>) {
        let id = borrowable.id;
        self.pool.push(borrowable);
        self.free_list.push(id);
        remove_from_list(&mut self.inuse_list, id);
    }

    fn print_free_list(&mut self) {
        println!("free list:{:?}", self.free_list);
        println!("in use list:{:?}", self.inuse_list);
    }
}
