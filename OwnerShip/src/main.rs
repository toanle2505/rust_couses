fn main() {
    println!("Hello, world!");
    // Ví dụ về Ownership trong Rust
    let s1 = String::from("hello"); // s1 là chủ sở hữu của chuỗi này
    let s2 = s1; // Ownership của chuỗi được chuyển sang s2, s1 không còn hợp lệ
    // println!("{}", s1); // Dòng này sẽ báo lỗi vì s1 đã bị move
    println!("{}", s2); // s2 là chủ sở hữu mới

    // Để giữ lại giá trị, ta có thể clone
    let s3 = String::from("world");
    let s4 = s3.clone(); // s3 và s4 đều sở hữu các bản sao riêng biệt
    println!("{} {}", s3, s4);

    // Ví dụ ownership với hàm
    let s5 = String::from("function");
    takes_ownership(s5); // s5 bị move vào hàm, không còn dùng được sau đây
    // println!("{}", s5); // Dòng này sẽ lỗi vì s5 đã bị move

    let s6 = String::from("return");
    let s7 = gives_ownership(s6); // s6 bị move vào hàm, ownership trả về cho s7
    println!("{}", s7); // s7 là chủ sở hữu mới 

    // Ví dụ không bị lấy ownership khi truyền vào hàm (mượn - borrowing)
    let s8 = String::from("borrow");
    print_borrow(&s8); // Truyền tham chiếu, ownership không bị lấy
    println!("Sau khi mượn bất biến: {}", s8); // s8 vẫn dùng được

    let mut s9 = String::from("mutable borrow");
    modify_borrow(&mut s9); // Truyền tham chiếu có thể thay đổi
    println!("Sau khi mượn có thể thay đổi: {}", s9); // s9 đã bị thay đổi
}

fn takes_ownership(s: String) {
    println!("Hàm nhận ownership: {}", s);
    // s sẽ bị drop khi ra khỏi scope
}

fn gives_ownership(s: String) -> String {
    println!("Hàm nhận và trả lại ownership: {}", s);
    s // trả ownership về cho người gọi
}

fn print_borrow(s: &String) {
    println!("Mượn bất biến: {}", s);
    // Không thể thay đổi giá trị của s
}

fn modify_borrow(s: &mut String) {
    s.push_str(" - đã thay đổi");
    println!("Mượn có thể thay đổi: {}", s);
}

// Giải thích:
// - Khi gán s1 cho s2, ownership của chuỗi được chuyển sang s2, s1 không còn hợp lệ.
// - Nếu muốn cả hai biến đều dùng được, cần dùng clone để tạo bản sao dữ liệu.
// - Khi biến sở hữu đi ra khỏi scope, bộ nhớ sẽ được giải phóng tự động.
    // - Khi truyền biến vào hàm, ownership có thể bị chuyển vào hàm đó. Biến bên ngoài không còn hợp lệ nếu ownership đã bị chuyển.
// - Hàm có thể trả về ownership cho biến khác, cho phép tiếp tục sử dụng giá trị bên ngoài scope của hàm.

// Ownership is a concept in Rust that ensures memory safety without needing a garbage collector. 
// It revolves around three main rules:
// 1. Each value in Rust has a single owner.
// 2. When the owner goes out of scope, the value is dropped.
// 3. A value can be borrowed either mutably or immutably, but not both at the same time.

