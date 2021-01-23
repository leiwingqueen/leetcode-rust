#[derive(Debug)]  // 链表节点(结点??)
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
struct Link {
    // 链表
    len: u32,
    node: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Self {     // . .. . 构造器?
        Node { data, next: None }
    }

    fn append_node(&mut self, node: Node) {  // ... 追加节点
        match &mut self.next {
            Some(ref mut t) => {
                t.append_node(node);
            }
            None => {
                self.next = Some(Box::new(node));
            }
        }
    }
    fn to_string(&self) -> String {  // 什么? 你要让我实现 fmt::Display ? 我就不
        match &self.next {
            Some(ref t) => format!("{} {}", self.data, t.to_string()),
            None => String::from(&self.data.to_string()),
        }
    }
}

impl Link {
    fn new() -> Self {
        Link { len: 0, node: None }
    }
    fn append_data(&mut self, data: i32) {  // 向后添加..
        let new_node = Node::new(data);
        match self.node {
            Some(ref mut t) => {
                t.append_node(new_node);
            }
            None => {
                self.node = Some(Box::new(new_node));
            }
        }
        self.len += 1;
    }
    fn to_string(&self) -> String {
        match self.node {
            Some(ref t) => format!("[{}]", t.to_string()),
            None => String::new(),
        }
    }

    fn remove_data(&mut self, index: u32) -> i32 {   // 删除
        let mut result = -1;
        let mut count = 0;
        let mut node = &mut self.node;
        while let Some(ref mut current) = node {
            count += 1;
            if count >= index {
                let old_node = current.next.take();
                match old_node {
                    Some(other_node) => {
                        result = other_node.data;
                        //let t = (*other_node).next;
                        let t = (other_node).next;
                        current.next = t;
                    }
                    None => {
                        current.next = None;
                    }
                }
                self.len -= 1;
                break;
            }
            node = &mut current.next;
        }

        return result;
    }
    fn update_data(&mut self, index: u32, new_data: i32) -> i32 {  // 更新/修改
        let mut result = -1;
        let mut count = 0;
        let mut node = &mut self.node;
        while let Some(ref mut current) = node {
            if count >= index {
                result = current.data;
                current.data = new_data;
                break;
            }
            count += 1;
            node = &mut current.next;
        }
        return result;
    }
    fn find_data(&self, find_data: i32) -> i32 {  // 查. . .
        let mut result = -1;
        let mut node = &self.node;
        let mut count = 0;
        while let Some(ref current) = node {
            if current.data == find_data {
                result = count;
                break;
            }
            count += 1;
            node = &current.next;
        }
        return result;
    }
}

#[test]
fn test() {
    let mut link = Link::new();
    link.append_data(1);
    link.append_data(2);
    link.append_data(3);
    link.append_data(4);
    link.append_data(5);
    println!("link = {}", link.to_string());
    let mut return_data = link.update_data(2, 43);
    println!("update result_data = {}", return_data);
    println!("link = {}", link.to_string());
    return_data = link.remove_data(3);
    println!("remove result_data = {}", return_data);
    println!("link = {}", link.to_string());
}
