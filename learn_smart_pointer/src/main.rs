// 智能指针通常使用struct实现，并且实现了
// - Deref Drop 两个 trait
// 1. Deref Trait：允许智能指针struct的实例像引用一样使用
// 实现 Deref Trait 使我们可以自定义解引用 * 运算符的行为
// 使用 DerefMut Trait 重载可变引用解引用 * 运算符
// 在类型和trait在下列三种情况发生时，Rust会执行 deref coercion
// 当 T: Deref<Target=U>，允许 &T 转换为 &U
// 当 T: DerefMut<Target=U>，允许 &mut T 转换为 &mut U
// 当 T: Deref<Target=U>，允许 &mut T 转换为 &U
// 2. Drop Trait：允许自定义 当智能指针实例走出作用域时 的代码

// 标准库中常见的智能指针：
// 1. Box<T>：在heap内存上分配值
// Box创建的指针一定是栈指向堆，引用则有可能栈指向栈

// 2. Rc<T>：启用多重所有权的引用计数类型
// 3. Ref<T> 和 RefMut<T>，通过 RefCell<T>访问：在运行时而不是编译时强制借用规则的类型

// 内部可变模式：不可变类型暴露出可修改其内部值的 API
// 引用循环：它们如何泄露内存，以及如何防止发生

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello: {}", name);
}

fn main() {
    // 直接声明 let b = 5; 此时数据存储在栈上
    // 若声明 Box::new(5) 则数据 5 存储在heap上，b为栈上指向该数据的指针
    // let b = Box::new(5);
    // println!("b = {}", b);

    // Box<T>:
    // 指针大小不会基于它指向的数据大小变化而变化
    // 若 T 类型的数据大小不确定，使用 Box<T> 包装后数据大小就能够确定了
    // 因此常常能够用Box来获得确定大小的递归类型，比如：链表节点类型
    // Box没有性能开销，只提供了“间接”存储的功能，同时实现Deref trait和Drop trait

    // let mut chain_list = ChainList::new(1);
    // chain_list.insert_after(2);
    // chain_list.insert_after(3);
    // chain_list.insert_after(4);
    // chain_list.insert_after(123);
    let mut chain_list = ChainList::new(String::from("hello"));
    chain_list.insert_after(String::from("world"));
    chain_list.insert_after(String::from("this"));
    chain_list.insert_after(String::from("code"));
    chain_list.insert_after(String::from("Rust"));
    if let Some(current_value) = chain_list.get_current_value() {
        println!("current_value = {}", current_value);
    }
    if let Some(next_value) = chain_list.get_next_value() {
        println!("next_value = {}", next_value);
    }
    if let Some(some_value) = chain_list.get_value_by_distance(3) {
        println!("some_value = {}", some_value);
    }

    let x = 5;
    let y = Box::new(x); // let y = &x;
    let z = MyBox::new(x);
    assert_eq!(x, *y);
    assert_eq!(x, *z); // *z == 自定义解引用行为 ==> *(z.deref())

    let m = MyBox::new(String::from("Rust"));

    // 当类型不匹配，则进行自动隐式解引用
    // &m ===> &MyBox<String>
    // 不匹配，自动隐式解引用：&(*(m.deref())) ===> &(*(&String)) ===> &String
    // 不匹配，继续：&(*(String.deref())) ===> &str
    hello(&m);
    hello("Rust");
}

enum ChainListNode<ChainNode> {
    Node(ChainNode),
    Nil,
}

struct ChainList<T>
    where T: Clone,
{
    value: T,
    next: ChainListNode<Box<ChainList<T>>>
}

impl<T: Clone> ChainList<T> {
    fn new(value: T) -> ChainList<T> {
        ChainList {
            value,
            next: ChainListNode::Nil,
        }
    }

    fn next(&self) -> Option<&ChainList<T>> {
        match &self.next {
            ChainListNode::Nil => None,
            ChainListNode::Node(next_node) => {
                Some(next_node)
            }
        }
    }

    fn insert_after(&mut self, value: T) {
        let mut new_node = Box::new(ChainList::new(value));

        match &self.next {
            ChainListNode::Nil => {
                self.next = ChainListNode::Node(new_node);
            },
            ChainListNode::Node(_) => {
                let ref mut next_node = self.next;
                new_node.next = std::mem::replace(next_node, ChainListNode::Nil);
                self.next = ChainListNode::Node(new_node);
            }
        }
    }

    // fn insert_to_last(&mut self, value: T) {
    //     let mut current_node = self;

    //     loop {
    //         match &mut current_node.next {
    //             ChainListNode::Nil => {
    //                 let new_node = Box::new(ChainList::new(value));
    //                 let ref mut next_node = current_node.next;
    //                 let _ = std::mem::replace(next_node, ChainListNode::Node(new_node));
    //                 break;
    //             },
    //             ChainListNode::Node(next_node) => {
    //                 current_node = &mut **next_node;
    //             }
    //         }
    //     }
    // }

    fn get_current_value(&self) -> Option<T> {
        Some(self.value.clone())
    }

    fn get_next_value(&self) -> Option<T> {
        match &self.next {
            ChainListNode::Nil => {
                None
            },
            ChainListNode::Node(next_node) => Some(next_node.value.clone()),
        }
    }

    fn get_value_by_distance(&self, mut distance: u32) -> Option<T> {
        if distance == 0 {
            self.get_current_value()
        } else {
            let mut node = get_value_from_options(self.next()).unwrap();
            loop {
                distance = distance - 1;
                if distance == 0 {
                    break;
                }
                node = get_value_from_options(node.next()).unwrap();
            }
            Some(node.value.clone())
        }
    }
}

pub fn get_value_from_options<T>(options: Option<T>) -> Result<T, &'static str> {
    if let Some(value) = options {
        Ok(value)
    } else {
        Err("Enum value is None!")
    }
}