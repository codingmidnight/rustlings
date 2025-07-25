use std::time::Instant;

fn arithmetic_operations() -> u64 {
    let mut result = 0u64;
    for i in 0..10_000_000 {
        result = result.wrapping_add(i);  // 使用wrapping_add避免panic
    }
    result
}

fn main() {
    let start = Instant::now();
    let result = arithmetic_operations();
    let duration = start.elapsed();
    
    println!("结果: {}", result);
    println!("耗时: {:?}", duration);
    
    // 测试不同的溢出处理方式
    let x: u8 = 255;
    
    // 1. 直接加法（release模式无检查，debug模式有检查）
    let y1 = x.wrapping_add(1);
    println!("wrapping_add: {}", y1);
    
    // 2. 检查加法
    if let Some(y2) = x.checked_add(1) {
        println!("checked_add: {}", y2);
    } else {
        println!("checked_add: 溢出!");
    }
    
    // 3. 饱和加法
    let y3 = x.saturating_add(1);
    println!("saturating_add: {}", y3);
} 